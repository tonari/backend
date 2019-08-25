//! Handles a set facility request.

use rocket::post;
use rocket_contrib::{
    databases::mongodb::{bson, doc},
    json,
    json::{Json, JsonValue},
};
use serde::Deserialize;

use super::insert_json_flattened;
use crate::{
    database::FacilityCollection,
    facilities::{IDPair, MinimalFacilityData, OperationResult},
};

/// Represents the data that can be set in the `set-facility`-API.
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub(in crate::facilities) struct SetFacilityData {
    /// Sets the name.
    name: Option<String>,
    /// Sets the value of the address.
    address: Option<serde_json::Value>,
    /// Sets the accessibility information.
    accessibility: Option<serde_json::Value>,
    /// The ID of the facility, if an existing facility is set.
    id: Option<IDPair>,
    /// The latitude of the facility.
    lat: f64,
    /// The longitude of the facility.
    lon: f64,
    /// Indicates whether a new facility is created.
    createNewFacility: bool,
}

/// Adds or modifies an facility in the facilities collection.
#[post("/set-facility", format = "application/json", data = "<data>")]
#[allow(non_snake_case)]
pub(in crate::facilities) fn set_facility(
    data: Json<SetFacilityData>,
    collection: FacilityCollection,
) -> JsonValue {
    let mut set_document = doc! {};

    let SetFacilityData {
        name,
        address,
        accessibility,
        id,
        lat,
        lon,
        createNewFacility,
    } = data.into_inner();

    /// Insert an optional value into a document.
    macro_rules! insert_into_doc {
        ($doc:expr, $prefix:expr, $name:ident, flatten = true) => {
            if let Some($name) = $name {
                // Note that we need to flatten the JSON here, because otherwise the following could happen
                // Original data:
                // { "address": { "city": "Berlin" } }
                //
                // We try to set:
                // { "address": { "street": "Friedrichstraße" } }
                //
                // But then the whole address field would be overwritten with the new value, so instead we want to set
                // { "address.street": "Friedrichstraße" }
                //
                // Finally resulting in:
                // { "address": { "city": "Berlin", "street": "Friedrichstraße" } }
                insert_json_flattened(&mut $doc, concat!($prefix, stringify!($name)), $name);
            }
        };
        ($doc:expr, $prefix:expr, $name:ident, flatten = false) => {
            if let Some($name) = $name {
                $doc.insert(concat!($prefix, stringify!($name)), $name);
            }
        };
    }

    if !createNewFacility {
        if let Some(IDPair {
            sourceId,
            originalId,
        }) = id
        {
            insert_into_doc!(set_document, "properties.", name, flatten = false);
            insert_into_doc!(set_document, "properties.", address, flatten = true);
            insert_into_doc!(set_document, "properties.", accessibility, flatten = true);

            let insert_result = collection.find_one_and_update(
                doc! { "properties.sourceId": sourceId.clone(), "properties.originalId": originalId.clone() },
                doc! { "$set": set_document },
                Some(MinimalFacilityData {
                    lat,
                    lon,
                    sourceId,
                    originalId,
                }),
            );

            match insert_result {
                Ok(_) => json!({ "result": OperationResult::success }),
                Err(_) => json!({ "result": OperationResult::failure }),
            }
        } else {
            json!({ "result": OperationResult::failure, "reason": "The `sourceId` and `originalId` inside `id` are required to update existing entries." })
        }
    } else {
        let mut props = doc! {};

        insert_into_doc!(props, "", name, flatten = false);
        insert_into_doc!(props, "", address, flatten = false);
        insert_into_doc!(props, "", accessibility, flatten = false);

        let document = doc! {
            "geometry": {
                "coordinates": [lon, lat],
                "type": "Point"
            },
            "properties": props
        };

        let insert_result = collection.insert(document);

        match insert_result {
            Ok(_) => json!({ "result": OperationResult::success }),
            Err(_) => json!({ "result": OperationResult::failure }),
        }
    }
}
