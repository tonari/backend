//! Handles interfacing with the database.

use chrono::Utc;
use rocket::{
    request::{self, FromRequest, Request},
    Outcome, Rocket,
};
use rocket_contrib::{
    database,
    databases::mongodb::{
        self, bson,
        coll::{
            options::{FindOneAndUpdateOptions, FindOptions},
            results::InsertOneResult,
            Collection,
        },
        db::ThreadedDatabase,
        doc,
        oid::ObjectId,
        to_bson, Bson, Client, Document, ThreadedClient,
    },
};

use crate::{
    configuration::INITIALIZE_DB,
    facilities::{IDPair, MinimalFacilityData},
};

/// Initializes the database.
///
/// This sets up needed invariants in the database, such as indices.
pub fn init(rocket: &mut Rocket) {
    let connection_info = rocket
        .config()
        .get_table("databases")
        .expect("No database configured. Please check out the README.md for information on how to fix this.")
        .get("sanitary_facilities")
        .expect("No database configured. Please check out the README.md for information on how to fix this.")
        .get("url")
        .expect("No database configured. Please check out the README.md for information on how to fix this.")
        .as_str()
        .expect("Invalid database connection string.");

    let client =
        Client::with_uri(connection_info).expect("Database connection could not be established.");

    let facilities_collection = client
        .db(&*crate::configuration::DATABASE_NAME)
        .collection(&*crate::configuration::FACILITIES_COLLECTION_NAME);

    if *INITIALIZE_DB > 0 {
        // Set up an index for the locations in the `facilities` collection.
        facilities_collection
            .create_index(doc! { "geometry": "2dsphere" }, None)
            .expect("Could not create a required index in the database.");
    }
}

/// Specifies the database connection type.
///
/// Using this type as a rocket request guard, allows for use of
/// database connections in handlers.
#[database("sanitary_facilities")]
pub struct DatabaseConnection(mongodb::db::Database);

/// A request guard for the facilities collection.
///
/// This is a short hand for retrieving a database connection and then
/// getting the collection from that.
pub struct FacilityCollection(mongodb::coll::Collection);

impl<'a, 'r> FromRequest<'a, 'r> for FacilityCollection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<FacilityCollection, ()> {
        let database_connection = DatabaseConnection::from_request(request)?;

        Outcome::Success(FacilityCollection(
            database_connection
                .client
                .db(&crate::configuration::DATABASE_NAME)
                .collection(&crate::configuration::FACILITIES_COLLECTION_NAME),
        ))
    }
}

impl FacilityCollection {
    /// Performs the given query on the facility collection returning all results in json.
    pub fn perform_json_query(
        &self,
        filter: Option<Document>,
    ) -> mongodb::Result<impl Iterator<Item = serde_json::Value>> {
        perform_json_query(&self.0, filter, None).map(|values| {
            values.map(|mut val| {
                if let Some(obj) = val.as_object_mut() {
                    // return `{ "properties": { "_id": <id> } }` instead of `{ "_id": { "$oid": <id> } }`, just like in the accessibility cloud API
                    let id = obj
                        .remove("_id")
                        .expect("MongoDB document doesn't contain `_id`.");

                    obj.entry("properties")
                        .or_insert_with(|| serde_json::json!({}))["_id"] = id["$oid"].clone();

                    let mut filter_flagged_content = |content: &str| {
                        if let Some(nested_property) = obj
                            .get_mut("properties")
                            .and_then(|props| props.as_object_mut())
                            .and_then(|props_obj| props_obj.get_mut(content))
                            .and_then(|images| images.as_array_mut())
                        {
                            *nested_property = nested_property
                                .drain(..)
                                .filter(|prop| {
                                    prop.as_object()
                                        .and_then(|prop_obj| prop_obj.get("flagged"))
                                        .and_then(|flagged| flagged.as_bool())
                                        != Some(true)
                                })
                                .collect();
                        }
                    };

                    filter_flagged_content("images");

                    filter_flagged_content("comments");
                }

                val
            })
        })
    }

    /// Performs the given query and returns the data without modifications.
    ///
    /// Note that this should not be used for client-facing data.
    pub fn find_raw(
        &self,
        filter: Option<Document>,
    ) -> mongodb::Result<impl Iterator<Item = serde_json::Value>> {
        perform_json_query(&self.0, filter, None)
    }

    /// Returns a facility by ID.
    pub fn by_id(&self, id: IDPair) -> mongodb::Result<Option<serde_json::Value>> {
        self.find_raw(Some(doc! {
            "properties.sourceId": id.sourceId,
            "properties.originalId": id.originalId,
        }))
        .map(|mut iter| iter.next())
    }

    /// Finds the first facility that matches and updates it according to the update document.
    pub fn find_one_and_update(
        &self,
        filter: Document,
        mut update: Document,
        upsert_data: Option<MinimalFacilityData>,
    ) -> mongodb::Result<Option<Document>> {
        let upsert = upsert_data.is_some();

        if let Some(facility_info) = upsert_data {
            let old_document = update.insert(
                "$setOnInsert",
                doc! {
                    "type": "Feature",
                    "geometry.type": "Point",
                    "geometry.coordinates": [facility_info.lon, facility_info.lat],
                    "properties.category": "toilets"
                },
            );

            assert!(old_document.is_none());
        }

        if let Some(set_doc) = update.get_mut("$set").and_then(|doc| match doc {
            Bson::Document(set_doc) => Some(set_doc),
            _ => None,
        }) {
            set_doc.insert("lastUpdated", Utc::now().to_string());
        } else {
            update.insert("$set", doc! { "lastUpdated": Utc::now().to_string() });
        }

        find_one_and_update(&self.0, filter, update, upsert)
    }

    /// Inserts a new facility into the collection.
    pub fn insert(&self, mut doc: Document) -> mongodb::Result<InsertOneResult> {
        let id = ObjectId::new()?;
        let id_str = id.to_string();

        doc.insert("_id", id);
        doc.insert("type", "Feature");
        doc.insert("lastUpdated", Utc::now().to_string());

        if doc.get_document("properties").is_err() {
            doc.insert("properties", doc! {});
        }
        let props = match doc.get_mut("properties").unwrap() {
            Bson::Document(doc) => doc,
            _ => unreachable!(),
        };

        props.insert("sourceId", &*crate::configuration::SOURCE_ID);
        props.insert("originalId", id_str);
        props.insert("category", "toilets");

        self.0.insert_one(doc, None)
    }
}

/// Performs the given query on the given collection returning all results in json.
fn perform_json_query(
    collection: &Collection,
    filter: Option<Document>,
    options: Option<FindOptions>,
) -> mongodb::Result<impl Iterator<Item = serde_json::Value>> {
    Ok(collection
        .find(filter, options)?
        .filter_map(|val| val.ok())
        .filter_map(|val| to_bson(&val).ok())
        .map(|val| val.into()))
}

/// Finds the first element that matches and updates it according to the update document.
fn find_one_and_update(
    collection: &Collection,
    filter: Document,
    update: Document,
    upsert: bool,
) -> mongodb::Result<Option<Document>> {
    let mut options = FindOneAndUpdateOptions::new();
    options.upsert = Some(upsert);

    collection.find_one_and_update(filter, update, Some(options))
}
