//! Contains routes for updating facility data.

pub(super) mod comments;
pub(super) mod ping_notification;
pub(super) mod set_facility;
pub(super) mod verify_attributes;
pub(super) mod will_visit;

use flats::{flatten_value, Scalar};
use rocket_contrib::databases::mongodb::Document;

use super::{IDPair, OperationResult};

/// Inserts the entry into the document in flattened form.
///
/// Flattened means that nested documents are all represented at the top level.
/// For example:
///
/// ```json
/// {
///     "name": "John Doe",
///     "address": {
///         "city": "nyc"
///     },
///     "phones": [
///     "+44 1234567",
///     "+44 2345678"
///     ]
/// }
/// ```
///
/// is converted to
///
/// ```json
/// {
///     "name": "John Doe",
///     "address.city": "nyc",
///     "phones.0": "+44 1234567",
///     "phones.1": "+44 2345678"
/// }
/// ```
fn insert_json_flattened(doc: &mut Document, prepend: &str, entry: serde_json::Value) {
    for (key, value) in flatten_value(entry) {
        let key = format!("{}.{}", prepend, key.replace('[', ".").replace(']', ""));
        match value {
            Scalar::Bool(val) => doc.insert(key, val),
            Scalar::Number(val) => doc.insert(key, serde_json::json!(val)),
            Scalar::String(val) => doc.insert(key, val),
            Scalar::Null => doc.insert(key, serde_json::Value::Null),
        };
    }
}
