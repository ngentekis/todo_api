use crate::routes::todo::Todo;

pub fn get_todo() -> Vec<Todo> {
    
    let desc = String::from("hello");
    let to_do = Todo { 
        desc
    };

    vec![to_do]
}