use diesel::prelude::*;
use serde::Serialize;
use crate::schema::todo;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = todo)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub td_desc: String,
    pub td_status: bool
}

#[derive(Insertable)]
#[diesel(table_name = todo)]
pub struct NewTodo<'a> {
    pub td_desc: &'a str
}
