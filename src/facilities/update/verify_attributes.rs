//! Handles a request to verify an attribute.

use rocket::post;
use rocket_contrib::{
    databases::mongodb::{bson, doc},
    json,
    json::{Json, JsonValue},
};
use serde::Deserialize;

use crate::{
    database::FacilityCollection,
    facilities::{attributes::ATTRIBUTES, IDPair, MinimalFacilityData, OperationResult},
};

/// The data required to verify an attribute.
#[derive(Deserialize)]
pub(in crate::facilities) struct VerifyAttributeData {
    /// The ID of the facility to verify attributes for.
    id: IDPair,
    /// The attributes to verify.
    attributes: Vec<String>,
    /// The latitude of the facility.
    lat: f64,
    /// The longitude of the facility.
    lon: f64,
}

/// Verifies an attribute of a facility.
#[post("/verify-attributes", format = "application/json", data = "<data>")]
#[allow(non_snake_case)]
pub(in crate::facilities) fn verify_attributes(
    data: Json<VerifyAttributeData>,
    collection: FacilityCollection,
) -> JsonValue {
    let VerifyAttributeData {
        id: IDPair {
            sourceId,
            originalId,
        },
        attributes,
        lat,
        lon,
    } = data.into_inner();

    // Filter attributes based on whether they're known to the server.
    let known_attributes: Vec<_> = attributes
        .iter()
        .filter(|attribute| {
            ATTRIBUTES
                .iter()
                .any(|attribute_description| attribute == &attribute_description.get_name())
        })
        .map(|attribute| bson!(attribute))
        .collect();
    let unknown_attributes: Vec<_> = attributes
        .iter()
        .filter(|attribute| {
            !ATTRIBUTES
                .iter()
                .any(|attribute_description| attribute == &attribute_description.get_name())
        })
        .collect();

    let insert_result = collection.find_one_and_update(
        doc! {
            "properties.sourceId": sourceId.clone(),
            "properties.originalId": originalId.clone()
        },
        doc! { "$addToSet": { "properties.verifiedAttributes": { "$each": known_attributes } } },
        Some(MinimalFacilityData {
            sourceId,
            originalId,
            lat,
            lon,
        }),
    );

    match insert_result {
        Ok(_) => {
            json!({ "result": OperationResult::success, "unknownAttributes": unknown_attributes })
        }
        Err(_) => {
            json!({ "result": OperationResult::failure, "unknownAttributes": unknown_attributes })
        }
    }
}
