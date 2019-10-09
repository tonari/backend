#![feature(proc_macro_hygiene, decl_macro)]
#![feature(drain_filter)]

mod configuration;
mod database;
mod facilities;
mod images;
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

    unsafe {
        signal_hook::register(signal_hook::SIGINT, || {
            println!("Received SIGINT signal.");
            std::process::exit(0);
        }).expect("signal handler could not be set");
    }

    let mut rocket = rocket::ignite()
        .attach(DatabaseConnection::fairing())
        .mount("/facilities", facilites_routes())
        .mount("/images", image_routes());

    if let Some(routes) = testpage_routes() {
        rocket = rocket.mount("/testpages", routes)
    }

    database::init(&mut rocket);

    rocket.launch();
}
