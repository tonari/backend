//! This modules deals with up- and downloading images.

use lazy_static::lazy_static;
use multipart::server::{save::PartialReason, Multipart, MultipartField, SaveResult};
use rocket::{
    data::DataStream,
    get,
    http::{
        hyper::mime::{Mime, SubLevel, TopLevel},
        ContentType, Status,
    },
    post,
    response::NamedFile,
    routes, Data, Route,
};
use rocket_contrib::{
    databases::mongodb::{bson, doc},
    json,
    json::{Json, JsonValue},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::remove_file,
    path::{Path, PathBuf},
};
use uuid::Uuid;

use crate::{
    database::FacilityCollection,
    facilities::{IDPair, MinimalFacilityData, OperationResult},
};

/// Represents the possible labels an image can have.
#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum ImageLabel {
    /// The image shows a toilet.
    toilet,
    /// The image shows the entry to a facility.
    entry,
    /// The image shows the sink of facility.
    sink,
    /// The image is taken from the entry of the room with the toilet.
    fromEntry,
    /// The image does not fit one of the previous types.
    other,
}

lazy_static! {
    /// The set of all possible image labels.
    ///
    /// Note that this set only contains labels that are meaningful on their own.
    ///
    /// `Other` for example does not really say anything about an image other than the fact that it does not
    /// fit any of the other labels. Collection images of type `Other` would be a useless task.
    pub static ref ALL_IMAGE_LABELS: HashSet<ImageLabel> = {
        let mut set = HashSet::with_capacity(5);

        set.insert(ImageLabel::toilet);
        set.insert(ImageLabel::entry);
        set.insert(ImageLabel::sink);
        set.insert(ImageLabel::fromEntry);

        set
    };
}

/// The type that represents an image ID.
pub type ImageID = Uuid;

/// Describes the result of uploading an image.
///
/// Note that this type only exists to make JSON serialization nicer. Otherwise a Rust `enum` would be used.
#[derive(Debug, Serialize)]
struct ImageUploadResult {
    /// The type of upload result.
    result: ImageUploadResultType,
    /// The ID of the uploaded image.
    id: Option<ImageID>,
}

impl ImageUploadResult {
    /// Creates a successful `ImageUploadResult`.
    fn success(id: ImageID) -> ImageUploadResult {
        ImageUploadResult {
            result: ImageUploadResultType::success,
            id: Some(id),
        }
    }

    /// Creates a failed `ImageUploadResult`.
    fn fail(reason: ImageUploadResultType) -> ImageUploadResult {
        ImageUploadResult {
            result: reason,
            id: None,
        }
    }
}

/// Describes an error that happens during an image upload.
#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
enum ImageUploadResultType {
    /// The image was uploaded successfully.
    success,
    /// The image file size is larger than the allowed maximum.
    tooLarge,
    /// The image isn't a jpeg file.
    ///
    /// Only jpeg files are allowed.
    notJpeg,
    /// Some internal error happened while saving.
    ///
    /// This is most likely an IO error.
    internalError,
}

/// The routes for handling images.
pub fn image_routes() -> Vec<Route> {
    routes![
        image_upload,
        image_download,
        set_image_label,
        flag_image,
        verify_image_label
    ]
}

/// Converts an image id into a path.
fn image_path_from_id(id: &ImageID) -> PathBuf {
    let mut image_path = PathBuf::from(&*crate::configuration::IMAGE_PATH);
    image_path.push(id.to_string());
    image_path.set_extension("jpg");

    image_path
}

/// Generates a new image id.
fn generate_image_id() -> ImageID {
    let id_in_use = |id| image_path_from_id(&id).exists();

    let mut id = Uuid::new_v4();

    while id_in_use(id) {
        id = Uuid::new_v4();
    }

    id
}

/// Generates an image url from the given image ID.
pub fn url_from_id(id: &ImageID) -> String {
    format!(
        "{}{}{}",
        &*crate::configuration::IMAGE_URL_PREFIX,
        id,
        &*crate::configuration::IMAGE_URL_SUFFIX
    )
}

/// Handles image downloads with the given id.
#[get("/<id>")]
fn image_download(id: rocket_contrib::uuid::Uuid) -> Option<NamedFile> {
    let id = id.into_inner();

    NamedFile::open(image_path_from_id(&id)).ok()
}

/// Handles uploading images.
#[post(
    "/upload/<sourceId>/<originalId>?<lat>&<lon>",
    format = "multipart/form-data",
    data = "<data>"
)]
#[allow(non_snake_case)]
fn image_upload(
    sourceId: String,
    originalId: String,
    lat: f64,
    lon: f64,
    data: Data,
    content_type: &ContentType,
    collection: FacilityCollection,
) -> Result<JsonValue, Status> {
    if !content_type.is_form_data() {
        return Err(Status::BadRequest);
    }

    let (_, boundary) = content_type
        .params()
        .find(|&(key, _)| key == "boundary")
        .ok_or(Status::BadRequest)?;

    let mut save_results = Vec::new();

    Multipart::with_body(data.open(), boundary)
        .foreach_entry(|entry| {
            if &*entry.headers.name == "image" {
                save_results.push(handle_image_upload(
                    entry,
                    &collection,
                    sourceId.clone(),
                    originalId.clone(),
                    lat,
                    lon,
                ));
            }
        })
        .map_err(|_| Status::InternalServerError)?;

    Ok(json!({ "results": save_results }))
}

/// Represents the post request that is used to set an image label.
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct SetImageLabelData {
    /// The URL of the image.
    imageURL: String,
    /// The label of the image to set.
    imageLabel: ImageLabel,
    /// The ID of the facility the image belongs to.
    id: IDPair,
    /// The latitude of the facility to label the image for.
    lat: f64,
    /// The longitude of the facility to label the image for.
    lon: f64,
}

/// Handles the request for setting an image label.
#[post("/set-label", format = "application/json", data = "<data>")]
#[allow(non_snake_case)]
fn set_image_label(data: Json<SetImageLabelData>, collection: FacilityCollection) -> JsonValue {
    let SetImageLabelData {
        imageURL,
        imageLabel,
        lat,
        lon,
        id: IDPair {
            sourceId,
            originalId,
        },
    } = data.into_inner();

    if !ALL_IMAGE_LABELS.contains(&imageLabel) {
        return json!({ "result": OperationResult::failure, "reason": "Unknown image label." });
    }

    let insert_result = collection.find_one_and_update(
        doc! { "properties.sourceId": sourceId.clone(), "properties.originalId": originalId.clone(), "properties.images.url": imageURL.clone() },
        doc! { "$set": { "properties.images.$.label": json!(imageLabel).as_str().unwrap() } },
        None,
    );

    match insert_result {
        Ok(None) => {
            // The image did not exist, so it must be a remote image. Insert it into the database.
            let insert_result = collection.find_one_and_update(
                doc! { "properties.sourceId": sourceId.clone(), "properties.originalId": originalId.clone() },
                doc! { "$push": { "properties.images": { "url": imageURL, "label": json!(imageLabel).as_str().unwrap() } } },
                Some(MinimalFacilityData {
                    sourceId: sourceId,
                    originalId: originalId,
                    lat: lat,
                    lon: lon
                }),
            );

            match insert_result {
                Ok(_) => json!({ "result": OperationResult::success }),
                Err(_) => json!({ "result": OperationResult::failure }),
            }
        }
        Ok(Some(_)) => json!({ "result": OperationResult::success }),
        Err(_) => json!({ "result": OperationResult::failure }),
    }
}

/// Represents the post request that is used to verify an image label.
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct VerifyImageLabel {
    /// The ID of the facility the image belongs to.
    id: IDPair,
    /// The URL of the image.
    imageURL: String,
    /// The label of the image.
    ///
    /// This makes sure that only images with the correct label can be verified.
    imageLabel: String,
}

/// Handles the request for setting an image label.
#[post("/verify-label", format = "application/json", data = "<data>")]
#[allow(non_snake_case)]
fn verify_image_label(data: Json<VerifyImageLabel>, collection: FacilityCollection) -> JsonValue {
    let VerifyImageLabel {
        id: IDPair {
            sourceId,
            originalId,
        },
        imageURL,
        imageLabel,
    } = data.into_inner();

    let insert_result = collection.find_one_and_update(
        doc! {
            "properties.sourceId": sourceId,
            "properties.originalId": originalId,
            "properties.images.url": imageURL.to_string(),
            "properties.images.label": imageLabel.to_string()
        },
        doc! { "$set": { "properties.images.$.labelVerified": true } },
        None,
    );

    match insert_result {
        Ok(Some(_)) => json!({ "result": OperationResult::success }),
        Ok(None) => json!({ "result": OperationResult::entryNotFound }),
        Err(_) => json!({ "result": OperationResult::failure }),
    }
}

/// Represents the data required to flag an image as inappropriate.
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct FlagImageData {
    /// The URL of the image to flag.
    imageURL: String,
    /// The ID of the facility the image belongs to.
    id: IDPair,
}

/// Handles the request for flagging an image as inappropriate.
#[post("/flag-image", format = "application/json", data = "<data>")]
#[allow(non_snake_case)]
fn flag_image(data: Json<FlagImageData>, collection: FacilityCollection) -> JsonValue {
    let FlagImageData {
        imageURL,
        id: IDPair {
            sourceId,
            originalId,
        },
    } = data.into_inner();

    let insert_result = collection.find_one_and_update(
        doc! { "properties.sourceId": sourceId, "properties.originalId": originalId, "properties.images.url": imageURL.to_string() },
        doc! { "$set": { "properties.images.$.flagged": true } },
        None,
    );

    match insert_result {
        Ok(Some(_)) => json!({ "result": OperationResult::success }),
        Ok(None) => json!({ "result": OperationResult::entryNotFound }),
        Err(_) => json!({ "result": OperationResult::failure }),
    }
}

/// Handle an image upload to the server.
fn handle_image_upload(
    mut image_entry: MultipartField<&mut Multipart<DataStream>>,
    collection: &FacilityCollection,
    source_id: String,
    original_id: String,
    lat: f64,
    lon: f64,
) -> ImageUploadResult {
    match image_entry.headers.content_type {
        Some(Mime(TopLevel::Image, SubLevel::Jpeg, _)) => (),
        _ => {
            return ImageUploadResult::fail(ImageUploadResultType::notJpeg);
        }
    }

    let id = generate_image_id();
    let path = image_path_from_id(&id);

    let save_result = image_entry
        .data
        .save()
        // Limit to files of size `IMAGE_UPLOAD_SIZE_LIMIT`.
        .size_limit(*crate::configuration::IMAGE_UPLOAD_SIZE_LIMIT)
        // Always save to disk.
        .memory_threshold(0)
        // Save to the given path.
        .with_path(&path);

    match save_result {
        SaveResult::Full(_) => {
            if !path_is_jpeg(&path) {
                // Delete the non-jpeg file, ignore if deleting fails.
                remove_file(&path).ok();

                return ImageUploadResult::fail(ImageUploadResultType::notJpeg);
            }

            let insert_result = collection.find_one_and_update(
                doc! { "properties.sourceId": source_id.clone(), "properties.originalId": original_id.clone() },
                doc! { "$push": { "properties.images": { "id": id.to_string(), "url": url_from_id(&id) } } },
                Some(MinimalFacilityData {
                    lat,
                    lon,
                    sourceId: source_id,
                    originalId: original_id,
                }),
            );

            match insert_result {
                Ok(_) => ImageUploadResult::success(id),
                Err(_) => {
                    // Delete the orphan file, ignore if deleting fails.
                    remove_file(&path).ok();

                    ImageUploadResult::fail(ImageUploadResultType::internalError)
                }
            }
        }
        SaveResult::Partial(_, reason) => {
            // Delete the partial file, ignore if deleting fails.
            remove_file(&path).ok();

            ImageUploadResult::fail(match reason {
                PartialReason::SizeLimit => ImageUploadResultType::tooLarge,
                _ => ImageUploadResultType::internalError,
            })
        }
        _ => ImageUploadResult::fail(ImageUploadResultType::internalError),
    }
}

/// Checks if the file at the given path is a jpeg file.
fn path_is_jpeg(path: &Path) -> bool {
    path.exists() && tree_magic::from_filepath(&path) == "image/jpeg"
}
