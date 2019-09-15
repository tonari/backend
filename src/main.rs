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
    configuration::{check_required_configuration, INITIALIZE_DB},
    database::DatabaseConnection,
    facilities::facilites_routes,
    images::image_routes,
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

    if *INITIALIZE_DB > 0 {
        database::init(&mut rocket);
    }

    rocket.launch();
}
