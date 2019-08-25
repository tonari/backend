//! Handles an add comment request.

use chrono::Utc;
use rocket::post;
use rocket_contrib::{
    databases::mongodb::{bson, doc},
    json,
    json::{Json, JsonValue},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    database::FacilityCollection,
    facilities::{IDPair, MinimalFacilityData, OperationResult},
};

/// The data to add a comment.
#[derive(Deserialize)]
pub(in crate::facilities) struct AddCommentData {
    /// The ID of the facility to add the comment to.
    id: IDPair,
    /// The content of the comment.
    content: String,
    /// The latitude of the facility.
    lat: f64,
    /// The longitude of the facility.
    lon: f64,
}

/// Adds a comment to a facility.
#[post("/add-comment", format = "application/json", data = "<data>")]
#[allow(non_snake_case)]
pub(in crate::facilities) fn add_comment(
    data: Json<AddCommentData>,
    collection: FacilityCollection,
) -> JsonValue {
    let AddCommentData {
        id: IDPair {
            sourceId,
            originalId,
        },
        content,
        lat,
        lon,
    } = data.into_inner();

    let id = Uuid::new_v4();

    let insert_result = collection.find_one_and_update(
        doc! {
            "properties.sourceId": sourceId.clone(),
            "properties.originalId": originalId.clone()
        },
        doc! { "$push": { "properties.comments": { "id": id.to_string(), "content": content, "timestamp": Utc::now().to_string() } } },
        Some(MinimalFacilityData {
            sourceId,
            originalId,
            lat,
            lon,
        }),
    );

    match insert_result {
        Ok(_) => json!({ "result": OperationResult::success }),
        Err(_) => json!({ "result": OperationResult::failure }),
    }
}

/// The data to flag a comment.
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub(in crate::facilities) struct FlagCommentData {
    /// The ID of the facility to add the comment to.
    id: IDPair,
    /// The content of the comment.
    commentId: Uuid,
}

/// Flags a comment as inappropriate.
#[post("/flag-comment", format = "application/json", data = "<data>")]
#[allow(non_snake_case)]
pub(in crate::facilities) fn flag_comment(
    data: Json<FlagCommentData>,
    collection: FacilityCollection,
) -> JsonValue {
    let FlagCommentData {
        id: IDPair {
            sourceId,
            originalId,
        },
        commentId,
    } = data.into_inner();

    let insert_result = collection.find_one_and_update(
        doc! { "properties.sourceId": sourceId, "properties.originalId": originalId, "properties.comments.id": commentId.to_string() },
        doc! { "$set": { "properties.comments.$.flagged": true } },
        None,
    );

    match insert_result {
        Ok(_) => json!({ "result": OperationResult::success }),
        Err(_) => json!({ "result": OperationResult::failure }),
    }
}
