use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

use crate::services::todo;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    pub desc: String,
}

#[get("/todo/get-todo")]
pub fn get_todo() -> Json<Vec<Todo>> {    
    Json(todo::get_todo())
}