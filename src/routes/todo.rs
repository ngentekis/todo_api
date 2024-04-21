use self::models::Todo;
use test_rocket::*;
use rocket::serde::json::Json;
use crate::services::todo::{self, get_all_todo};

#[get("/todo/get-todo")]
pub fn get_todo() -> Json<Vec<Todo>> {    
    let results = get_all_todo();
    
    Json(results)
}

#[post("/todo/add-todo", format = "json", data = "<td_desc>")]
pub fn add_todo(td_desc: String) {
    todo::add_todo(td_desc);
}

#[delete("/todo/add-todo/<id>")]
pub fn delete_todo(id: i32) {
    todo::delete_todo(id);
}