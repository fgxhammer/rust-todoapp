use std::env;

struct TodoItem {
    name: String,
    completed: char
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let new_todo = TodoItem{
        name: "Say hi!".to_string(),
        completed: ' '
    };
    let todo_list = vec![new_todo];

    if command == "get" {
        for todo in todo_list {
            println!("[{}] - {}", todo.completed, todo.name)
        }
    }
}