//! Handles a ping-notification request.

use rocket::post;
use rocket_contrib::{
    json,
    json::{Json, JsonValue},
};
use serde::Deserialize;
use web_push::SubscriptionInfo;

use super::{IDPair, OperationResult};
use crate::{
    database::FacilityCollection, facilities::questions::generate_facility_questions, notifications,
};

/// Represents the data sent by a will visit request.
#[derive(Deserialize)]
pub(in crate::facilities) struct PingNotificationData {
    /// The delay to wait before sending a notification.
    delay: u16,
    /// The ID of the facility to visit.
    id: IDPair,
    /// The subscription to use for the notification.
    subscription: SubscriptionInfo,
}

/// Indicates that the user will visit the specified location after the given search.
#[post("/ping-notification", format = "application/json", data = "<data>")]
pub(in crate::facilities) fn ping_notification(
    data: Json<PingNotificationData>,
    collection: FacilityCollection,
) -> JsonValue {
    let PingNotificationData {
        delay,
        id,
        subscription,
    } = data.into_inner();

    if let Ok(Some(facility)) = collection.by_id(id.clone()) {
        let message = serde_json::json!({
            "questions": generate_facility_questions(&facility),
            "id": id,
            "lat": facility["geometry"]["coordinates"][1],
            "lon": facility["geometry"]["coordinates"][0]
        });

        if notifications::add_to_queue(subscription, message, Some(delay)).is_ok() {
            json!({ "result": OperationResult::success })
        } else {
            json!({ "result": OperationResult::failure, "reason": "The notification could not be constructed." })
        }
    } else {
        json!({ "result": OperationResult::failure, "reason": "The given facility could not be found." })
    }
}
