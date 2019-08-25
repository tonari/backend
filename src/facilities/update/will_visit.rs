//! Handles a will-visit request.

use rocket::post;
use rocket_contrib::{
    json,
    json::{Json, JsonValue},
};
use serde::Deserialize;

use super::{IDPair, OperationResult};
use crate::{
    database::FacilityCollection,
    facilities::{query::perform_radius_search, questions::generate_facility_questions},
};

/// Represents a radius search.
#[derive(Deserialize)]
pub(in crate::facilities) struct RadiusSearch {
    /// The latitude of the search center.
    lat: f64,
    /// The longitude of the search center.
    lon: f64,
    /// The radius of the search.
    radius: f64,
}

/// Represents the data sent by a will visit request.
#[derive(Deserialize)]
pub(in crate::facilities) struct WillVisitData {
    /// The search that was performed.
    search: RadiusSearch,
    /// The ID of the facility to visit.
    id: IDPair,
}

/// Indicates that the user will visit the specified location after the given search.
#[post("/will-visit", format = "application/json", data = "<data>")]
pub(in crate::facilities) fn will_visit(
    data: Json<WillVisitData>,
    collection: FacilityCollection,
) -> JsonValue {
    let WillVisitData { search, id } = data.into_inner();

    let radius_search_results: Vec<serde_json::Value> =
        perform_radius_search(search.lon, search.lat, search.radius, &collection)
            .map(|iter| iter.collect())
            .unwrap_or_else(|| Vec::new());

    let index_in_search = radius_search_results
        .iter()
        .enumerate()
        .find(|(_, elem)| {
            elem["properties"]["sourceId"] == id.sourceId
                && elem["properties"]["originalId"] == id.originalId
        })
        .map(|(index, _)| index);

    let facility = if let Ok(Some(facility)) = collection.by_id(id) {
        Some(facility)
    } else {
        None
    };

    match (facility, index_in_search) {
        (Some(_), None) => {
            // The facility was in the database, but not in the search. This is most likely a bug in the frontend.
            json!({ "result": OperationResult::failure, "reason": "Facility is not in given search."})
        }
        (Some(facility), Some(_index_in_search)) => {
            json!({ "result": OperationResult::success, "questions": generate_facility_questions(&facility)})
        }
        _ => {
            // We don't know anything about the facility, so we assume nothing about it.
            let empty_facility = json!({});
            json!({ "result": OperationResult::success, "questions": generate_facility_questions(&empty_facility) })
        }
    }
}
