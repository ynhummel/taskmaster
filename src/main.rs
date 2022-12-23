mod task;

use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use task::Task;

fn main() {
    let action = env::args().nth(1).unwrap_or_default();
    let mut database = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open("database.txt")
        .unwrap();

    if action == "new" {
        let name = env::args().nth(2).unwrap();
        let description = env::args().nth(3).unwrap();
        let new_task = Task::new(name, description);
        if let Err(e) = writeln!(database, "{}", new_task) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    if action == "list" {
        let mut buffer = String::new();
        if let Err(e) = database.read_to_string(&mut buffer) {
            eprintln!("Couldn't read file: {}", e);
        }
        println!("{}", buffer);
    }
}
