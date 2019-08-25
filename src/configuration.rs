//! Handles the configuration of the server.
//!
//! To set a variable use an environment variable with the prefix `TONARI_` and then the variable name.
//! For example you can set the path where images are saved using.
//!
//! ```bash
//! TONARI_IMAGE_PATH=path/to/images path/to/executable
//! ```

#[macro_use]
mod macros;

use lazy_static::lazy_static;

configuration_variable! {
    /// The maximum size of an image upload.
    pub static ref IMAGE_UPLOAD_SIZE_LIMIT: u64 = 10 * 1024 * 1024;

    /// The path where all the uploaded images are stored.
    pub static ref IMAGE_PATH := {
        if cfg!(feature = "testpages") {
            "/tmp/backend_images"
        } else {
            panic!("You need to set the {} environment variable to the correct image path before you can run the backend.", env_var_name!(IMAGE_PATH))
        }
    };

    /// The name of the MongoDB database to use.
    pub static ref DATABASE_NAME := "sanitary_facilities";

    /// The name of the database collection for sanitary facilities.
    pub static ref FACILITIES_COLLECTION_NAME := "facilities";

    /// The path to the VAPID private key file.
    pub static ref VAPID_PRIVATE_FILE := "vapid/private.pem";

    /// The path to the VAPID public key file in form of a JS array.
    pub static ref VAPID_PUBLIC_JS := "vapid/public.js";

    /// The email address for contacting the server administrator.
    pub static ref CONTACT_EMAIL_ADDRESS := {
        if cfg!(feature = "testpages") {
            "test@example.com"
        } else {
            panic!("You need to set the {} environment variable to the correct contact email address before you can run the backend.", env_var_name!(CONTACT_EMAIL_ADDRESS))
        }
    };

    /// The source ID of our data in the accessibility cloud.
    pub static ref SOURCE_ID := {
        if cfg!(feature = "testpages") {
            "MY_AC_SOURCE_ID"
        } else {
            panic!("You need to set the {} environment variable to the correct source ID before you can run the backend.", env_var_name!(SOURCE_ID))
        }
    };

    /// The prefix to use for an image URL.
    pub static ref IMAGE_URL_PREFIX := {
        if cfg!(feature = "testpages") {
            "http://localhost:8000/images/"
        } else {
            panic!("You need to set the {} environment variable to the correct URL prefix before you can run the backend.", env_var_name!(IMAGE_URL_PREFIX))
        }
    };

    /// The suffix to use for an image URL.
    ///
    /// This could for example help when images are not served over this server and need an extension in their file names.
    pub static ref IMAGE_URL_SUFFIX := {
        ""
    };
}

/// Check that all required configuration variables are set.
pub fn check_required_configuration() {
    let _ = (
        &*SOURCE_ID,
        &*IMAGE_URL_PREFIX,
        &*CONTACT_EMAIL_ADDRESS,
        &*IMAGE_PATH,
    );
}
