//! Handles the generation and answering of questions regarding facilities.
#![allow(non_snake_case)]

use serde::Serialize;
use uuid::Uuid;

use crate::images::ImageLabel;

use crate::{
    facilities::attributes::ATTRIBUTES,
    images::{self, ALL_IMAGE_LABELS},
};

/// Represents a question to ask the user.
#[derive(Serialize)]
#[allow(non_camel_case_types)]
#[serde(tag = "type")]
pub enum Question {
    /// Asks the user to add an image.
    ///
    /// The image may optionally be of a certain type.
    addImage { imageLabel: ImageLabel },
    /// Asks the user to label an image.
    labelImage { imageURL: String },
    /// Asks the user to verify a label for an image.
    verifyLabel {
        imageURL: String,
        imageLabel: ImageLabel,
    },
    /// Asks the user about an attribute.
    askAttribute { attribute: String },
    /// Asks the user to verify the value of an attribute.
    verifyAttribute {
        attribute: String,
        value: serde_json::Value,
    },
}

/// Generates questions about a facility.
pub fn generate_facility_questions(facility: &serde_json::Value) -> Vec<Question> {
    let mut questions = Vec::new();

    generate_attribute_questions(&facility, &mut questions);
    generate_image_questions(&facility, &mut questions);

    questions
}

/// Generates all questions regarding attributes for the given facility.
fn generate_attribute_questions(facility: &serde_json::Value, questions: &mut Vec<Question>) {
    for attribute in &ATTRIBUTES {
        let name = attribute.get_name();
        let value = attribute.get_value(&facility);

        if let Some(verified_attributes) = facility["properties"]["verifiedAttributes"].as_array() {
            if verified_attributes
                .iter()
                .any(|attribute| attribute.as_str().unwrap_or("") == name)
            {
                continue;
            }
        }

        if let Some(value) = value {
            questions.push(Question::verifyAttribute {
                attribute: String::from(name),
                value,
            })
        } else {
            questions.push(Question::askAttribute {
                attribute: String::from(name),
            })
        }
    }
}

/// Generates all questions regarding images for the given facility.
fn generate_image_questions(facility: &serde_json::Value, questions: &mut Vec<Question>) {
    let empty_list = Vec::new();

    let images = facility["properties"]["images"]
        .as_array()
        .unwrap_or_else(|| &empty_list);

    let mut unused_labels = (*ALL_IMAGE_LABELS).clone();

    for image in images {
        if let Ok(id) = serde_json::from_value::<Uuid>(image["id"].clone()) {
            let url = image["url"]
                .as_str()
                .map(|url| url.to_string())
                .unwrap_or_else(|| images::url_from_id(&id));

            if let Ok(label) = serde_json::from_value::<ImageLabel>(image["label"].clone()) {
                unused_labels.remove(&label);

                if image["labelVerified"].as_bool() != Some(true) {
                    questions.push(Question::verifyLabel {
                        imageURL: url,
                        imageLabel: label,
                    });
                }
            } else {
                questions.push(Question::labelImage { imageURL: url });
            }
        }
    }

    for label in unused_labels {
        questions.push(Question::addImage { imageLabel: label })
    }
}
