#![feature(proc_macro_hygiene, decl_macro)]
#![feature(drain_filter)]

mod configuration;
mod database;
mod facilities;
mod images;
mod notifications;
#[cfg(feature = "testpages")]
mod testpages;

use rocket::Route;

use crate::{
    configuration::check_required_configuration, database::DatabaseConnection,
    facilities::facilites_routes, images::image_routes,
};

/// The routes for pages to test the features.
#[cfg(feature = "testpages")]
fn testpage_routes() -> Option<Vec<Route>> {
    Some(crate::testpages::testpage_routes())
}

/// The routes for pages to test the features.
#[cfg(not(feature = "testpages"))]
fn testpage_routes() -> Option<Vec<Route>> {
    None
}

/// The main entry point of the server.
fn main() {
    check_required_configuration();

    let mut rocket = rocket::ignite()
        .attach(DatabaseConnection::fairing())
        .mount("/facilities", facilites_routes())
        .mount("/images", image_routes());

    if let Some(routes) = testpage_routes() {
        rocket = rocket.mount("/testpages", routes)
    }

    let database_connection_info = rocket
        .config()
        .get_table("databases")
        .expect("No database configured. Please check out the README.md for information on how to fix this.")
        .get("sanitary_facilities")
        .expect("No database configured. Please check out the README.md for information on how to fix this.")
        .get("url")
        .expect("No database configured. Please check out the README.md for information on how to fix this.")
        .as_str()
        .expect("Invalid database connection string.");

    database::init(&database_connection_info);

    rocket.launch();
}
