use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem{
            name: name,
            completed: ' '
        };
    }
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList{list: Vec::new()}
    }

    fn add_to_list(&mut self, name: String) {
        self.list.push(TodoItem::new(name))
    }
}



fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let mut todo_list = TodoList::new();

    todo_list.add_to_list("Say hi!".to_string());
    todo_list.add_to_list("Say Bye!".to_string());

    if command == "get" {
        for todo in todo_list.list {
            println!("[{}] - {}", todo.completed, todo.name)
        }
    }
}