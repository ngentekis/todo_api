mod routes;
mod services;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
#[macro_use] extern crate rocket;

use routes::date::get_current_date;
use routes::todo::get_todo;

#[rocket::main]
async fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&["http://127.0.0.1:4200"]);

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
        .mount("/api", routes![get_current_date, get_todo])
        .attach(cors)
        .launch()
        .await
        .expect("Rocket failed to launch");
}