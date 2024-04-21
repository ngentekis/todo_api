pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{NewTodo, Todo};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_todo(conn: &mut PgConnection, td_desc: &str) -> Todo {
    use crate::schema::todo;

    let new_todo = NewTodo { td_desc };

    return diesel::insert_into(todo::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new todo item")    
}