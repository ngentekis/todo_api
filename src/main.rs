mod routes;
mod services;
use std::env;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
#[macro_use] extern crate rocket;
use dotenvy::dotenv;

use routes::date::get_current_date;
use routes::todo::{get_todo, add_todo, delete_todo};

#[rocket::main]
async fn main() {

    dotenv().ok();

    let origins_url = env::var("ALLOWED_ORIGINS").expect("ALLOWED_ORIGINS must be set");

    let allowed_origins = AllowedOrigins::some_exact(&[origins_url]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error while building CORS");

    rocket::build()
        .mount("/api", routes![get_current_date, get_todo, add_todo, delete_todo])
        .attach(cors)
        .launch()
        .await
        .expect("Rocket failed to launch");
}