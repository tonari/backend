//! This modules contains all the routes for querying facilities.

use geoutils::Location;
use rocket::get;
use rocket_contrib::{
    databases::mongodb::{bson, doc},
    json,
    json::JsonValue,
};
use slippy_map_tilenames::tile2lonlat;

use super::OperationResult;
use crate::database::FacilityCollection;

/// Returns all facilities in the specified map tile.
#[get("/by-tile/<x>/<y>/<z>")]
pub(super) fn by_tile(
    x: u32,
    y: u32,
    z: u8,
    collection: FacilityCollection,
) -> Result<JsonValue, JsonValue> {
    let (bottom_left_lon, bottom_left_lat) = tile2lonlat(x, y + 1, z);
    let (top_right_lon, top_right_lat) = tile2lonlat(x + 1, y, z);

    let features: Vec<serde_json::Value> = collection.perform_json_query(
        // Refer to https://docs.mongodb.com/manual/reference/operator/query/box/#op._S_box for more information about the specific query syntax
        Some(doc! { "geometry": { "$geoWithin": { "$box" : [[bottom_left_lon, bottom_left_lat], [top_right_lon, top_right_lat]] } } }),
    )
    .map_err(|_| json!({ "result": OperationResult::failure }))?
    .collect();

    Ok(
        json!({ "result": OperationResult::success, "features": features, "featureCount": features.len() }),
    )
}

/// Returns all facilities in the specified radius around the given coordinates.
///
/// The radius is given in meters.
#[get("/by-radius/<longitude>/<latitude>/<radius>")]
pub(super) fn by_radius(
    longitude: f64,
    latitude: f64,
    radius: f64,
    collection: FacilityCollection,
) -> Result<JsonValue, JsonValue> {
    if longitude > 180.0
        || longitude < -180.0
        || latitude > 90.0
        || latitude < -90.0
        || !radius.is_normal() // make sure the radius is a well behaved floating point number
        || radius <= 0.0
    {
        return Err(
            json!({ "result": OperationResult::failure, "reason": "The parameters were not in the legal range."}),
        );
    }

    let features: Vec<serde_json::Value> =
        perform_radius_search(longitude, latitude, radius, &collection)
            .ok_or_else(|| json!({ "result": OperationResult::failure }))?
            .collect();

    Ok(
        json!({ "result": OperationResult::success, "features": features, "featureCount": features.len() }),
    )
}

/// Performs a radius search for facilities.
pub fn perform_radius_search(
    longitude: f64,
    latitude: f64,
    radius: f64,
    collection: &FacilityCollection,
) -> Option<impl Iterator<Item = serde_json::Value>> {
    if longitude > 180.0
        || longitude < -180.0
        || latitude > 90.0
        || latitude < -90.0
        || !radius.is_normal() // make sure the radius is a well behaved floating point number
        || radius <= 0.0
    {
        return None;
    }

    let search_location = Location::new(latitude, longitude);

    Some(
        collection
            .perform_json_query(
                // Refer to https://docs.mongodb.com/manual/reference/operator/query/nearSphere/
                Some(doc! {
                    "geometry": {
                        "$nearSphere": {
                            "$geometry": {
                                "type": "Point",
                                "coordinates": [longitude, latitude]
                            },
                            "$maxDistance": radius
                        }
                    }
                }),
            )
            .ok()?
            // Add distance to the results and filter the results out where adding the distance is impossible
            .filter_map(move |mut val| {
                let coords = val.get("geometry")?.get("coordinates")?;
                let (lat, lon) = (coords.get(1)?.as_f64()?, coords.get(0)?.as_f64()?);
                let facility_location = Location::new(lat, lon);
                let distance = search_location
                    .distance_to(&facility_location)
                    .unwrap_or_else(|_| search_location.haversine_distance_to(&facility_location));

                if let Some(props) = val
                    .as_object_mut()?
                    .get_mut("properties")
                    .and_then(|props| props.as_object_mut())
                {
                    props.insert(String::from("distance"), distance.into());
                } else {
                    val.as_object_mut()?.insert(
                        String::from("properties"),
                        serde_json::json!({ "distance": distance }),
                    );
                }

                Some(val)
            }),
    )
}

/// Returns the facility with the given ID.
#[get("/by-id/<sourceId>/<originalId>")]
#[allow(non_snake_case)]
pub(super) fn by_id(
    sourceId: String,
    originalId: String,
    collection: FacilityCollection,
) -> Result<JsonValue, JsonValue> {
    collection
        .perform_json_query(Some(
            doc! { "properties.sourceId": sourceId, "properties.originalId": originalId },
        ))
        .map_err(|_| json!({ "result": OperationResult::failure }))?
        .next()
        .map(|val| json!({ "result": OperationResult::success, "features": [json!(val)], "featureCount": 1 }))
        .ok_or_else(|| json!({ "result": OperationResult::success, "features": [], "featureCount": 0 }))
}

/// Returns all facilities in the specified source.
#[get("/by-source-id/<sourceId>")]
#[allow(non_snake_case)]
pub(super) fn by_source_id(
    sourceId: String,
    collection: FacilityCollection,
) -> Result<JsonValue, JsonValue> {
    let features: Vec<serde_json::Value> = collection
        .perform_json_query(Some(doc! { "properties.sourceId": sourceId }))
        .map_err(|_| json!({ "result": OperationResult::failure }))?
        .collect();

    Ok(
        json!({ "result": OperationResult::success, "features": features, "featureCount": features.len() }),
    )
}

/// Returns all facilities that have been updated since the given timestamp.
#[get("/updated-since/<timestamp>?<source_id>")]
#[allow(non_snake_case)]
pub(super) fn updated_since(
    timestamp: String,
    source_id: Option<String>,
    collection: FacilityCollection,
) -> Result<JsonValue, JsonValue> {
    let mut query = doc! { "lastUpdated": { "$gte": timestamp } };

    if let Some(source_id) = source_id {
        query.insert("properties.sourceId", source_id);
    }

    let features: Vec<serde_json::Value> = collection
        .perform_json_query(Some(query))
        .map_err(|_| json!({ "result": OperationResult::failure }))?
        .collect();

    Ok(
        json!({ "result": OperationResult::success, "features": features, "featureCount": features.len() }),
    )
}
