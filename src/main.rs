mod cli;
mod task;

use clap::{Parser, Subcommand};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use task::Task;

#[derive(Parser)]
#[command(name = "TaskMaster")]
#[command(author = "ynh")]
#[command(version = "0.0.1")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: Option<String>,
        description: Option<String>,
    },
    List,
}

fn main() {
    let mut database = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open("database.txt")
        .unwrap();

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, description } => {
            let new_task = Task::new(name.unwrap(), description.unwrap());
            if let Err(e) = writeln!(database, "{}", new_task) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        Commands::List => {
            let mut buffer = String::new();
            if let Err(e) = database.read_to_string(&mut buffer) {
                eprintln!("Couldn't read file: {}", e);
            }
            println!("{}", buffer);
        }
    }
}
