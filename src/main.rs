mod models;
mod routes;
mod services;
mod utils;

use std::sync::Arc;

use mongodb::Client;
use rocket::tokio::sync::Mutex;
use rocket::{launch, routes};

pub struct AppState {
    mongo_client: Arc<Mutex<Client>>,
}

#[launch]
async fn rocket() -> _ {
    let mongo_client = Client::with_options(
        mongodb::options::ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap(),
    )
    .unwrap();

    rocket::build()
        .manage(AppState {
            mongo_client: Arc::new(Mutex::new(mongo_client)),
        })
        .mount(
            "/api/v1/",
            routes![
                routes::todo::index,
                routes::todo::create,
                routes::todo::update,
                routes::todo::delete,
                routes::user::login,
                routes::user::register
            ],
        )
}
