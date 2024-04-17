use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

use crate::services::date;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32
}

#[get("/date/get-current-date")]
pub fn get_current_date() -> Json<Date> {
    Json(date::get_current_date())
}