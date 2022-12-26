mod task;

use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
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
    Delete {
        name: Option<String>,
    },
}

fn main() -> Result<()> {
    let conn = Connection::open("./database.db3")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id             INTEGER PRIMARY KEY,
            name           TEXT NOT NULL,
            description    TEXT
        )",
        (),
    )?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name, description } => {
            let new_task = Task::new(name.unwrap(), description.unwrap());
            conn.execute(
                "INSERT INTO task (name, description) VALUES (?1, ?2)",
                (&new_task.name, &new_task.description),
            )?;
        }
        Commands::Delete { name } => {
            let filter_name = name.unwrap();
            let mut stmt = conn.prepare("DELETE FROM task WHERE (name) = (?1)")?;
            stmt.execute([filter_name])?;
        }
        Commands::List => {
            let mut stmt = conn.prepare("SELECT id, name, description FROM task")?;
            let task_iter = stmt.query_map([], |row| {
                Ok(Task {
                    name: row.get(1)?,
                    description: row.get(2)?,
                })
            })?;

            for task in task_iter {
                println!("{}", task.unwrap());
            }
        }
    }
    Ok(())
}
