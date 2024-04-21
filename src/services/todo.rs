use self::models::Todo;
use diesel::prelude::*;
use test_rocket::*;

pub fn get_all_todo() -> Vec<Todo> {
    
    use self::schema::todo::dsl::*;

    let connection = &mut establish_connection();
    let results = todo.
        select(Todo::as_select())
        .load(connection)
        .expect("Error loading todo items");

    results
}

pub fn add_todo(desc: String) {

    let connection = &mut establish_connection();

    create_todo(connection, &desc);    
}

pub fn delete_todo(todo_id: i32) {
    use self::schema::todo::dsl::*;

    let connection = &mut establish_connection();

    diesel::delete(todo.filter(id.eq(todo_id)))
        .execute(connection)
        .expect("Error deleting todo item");
}