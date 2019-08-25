//! Implements the API for retrieving information about facilities.

mod attributes;
mod query;
mod questions;
mod update;

use rocket::{routes, Route};
use serde::{Deserialize, Serialize};

/// Returns the routes of the facilities API.
pub fn facilites_routes() -> Vec<Route> {
    routes![
        query::by_tile,
        query::by_radius,
        query::by_id,
        query::by_source_id,
        query::updated_since,
        update::set_facility::set_facility,
        update::will_visit::will_visit,
        update::comments::add_comment,
        update::comments::flag_comment,
        update::verify_attributes::verify_attributes,
        update::ping_notification::ping_notification
    ]
}

/// Represents an ID for entries in the database.
#[derive(Deserialize, Serialize, Clone, Debug)]
#[allow(non_snake_case)]
pub struct IDPair {
    /// The ID for source the data in the database.
    pub sourceId: String,
    /// The ID of the data in the original source.
    pub originalId: String,
}

/// The result of applying a change.
#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum OperationResult {
    /// The change was successfully applied.
    success,
    /// Something went wrong while applying the change.
    failure,
    /// The facility or part of the facility to modify was not found.
    ///
    /// This is only used if creating a new facility is not applicable in this case.
    entryNotFound,
}

/// Represents the minimal amount of information that needs to be available about a facility.
#[allow(non_snake_case)]
pub struct MinimalFacilityData {
    /// The original ID of the facility.
    pub originalId: String,
    /// The accessibility cloud source ID of the original facility source.
    pub sourceId: String,
    /// The latitude of the facility.
    pub lat: f64,
    /// The longitude of the facility.
    pub lon: f64,
}
