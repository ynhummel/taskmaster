use std::env;
use std::fs::File;

fn main() {
    let action = env::args().nth(1).unwrap_or_default();
    let mut task_list: Vec<Task> = Vec::new();

    if action == "new" {
        let name = env::args().nth(2).unwrap();
        let description = env::args().nth(3).unwrap();
        let new_task = Task::new(name, description);
        task_list.push(new_task);
    }

    for task in task_list {
        println!("{:?}", task);
    }
}

#[derive(Debug)]
struct Task {
    name: String,
    description: String,
}

impl Task {
    fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}
