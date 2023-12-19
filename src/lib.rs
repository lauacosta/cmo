use anyhow::{Result, bail};
use std::fs::{File, OpenOptions};
use clap::{Parser, Subcommand};
use colored::Colorize;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority {
    HIGH,
    MEDIUM,
    LOW,
}
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::HIGH => write!(f, "High"),
            Priority::MEDIUM => write!(f, "Medium"),
            Priority::LOW => write!(f, "Low"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task: String,
    priority: Priority,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let task = match self.priority {
            Priority::LOW => format!(
                "\t[{}] - {}",
                self.priority.to_string().green().bold(),
                self.task.to_string().green().bold()
            ),
            Priority::MEDIUM => format!(
                "\t[{}] - {}",
                self.priority.to_string().yellow().bold(),
                self.task.to_string().yellow().bold()
            ),
            Priority::HIGH => format!(
                "\t[{}] - {}",
                self.priority.to_string().red().bold(),
                self.task.to_string().red().bold()
            ),
        };
        write!(f, "{task}")
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a task
    Add {
        /// Description of the task
        task: String,
        /// Priority Order of the task
        #[arg(short, long, default_value_t = String::from("low"))]
        priority: String,
    },
    /// Print the tasks present
    Print {},
    Remove {
        /// Description of the task
        task: String,
    },
}


fn load_tasks(file: File) -> Result<Vec<Task>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut result = Vec::new();
    for data in reader.deserialize() {
        let task: Task = data?;
        result.push(task);
    }

    result.sort_by_key(|k| k.priority);
    Ok(result)
}

pub fn remove_task(needle: String, file:File) -> Result<()> {
    todo!();
}

fn print_tasks(file: File) -> Result<()> {
    let record = load_tasks(file)?;
    for entry in record {
        println!("{entry}");
    }
    Ok(())
}

fn add_task(task: String, priority: &str, file: File) -> Result<()> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    let task = match priority {
        "low" | "l" => Task {
            task,
            priority: Priority::LOW,
        },
        "medium" | "m" => Task {
            task,
            priority: Priority::MEDIUM,
        },
        "high" | "h" => Task {
            task,
            priority: Priority::HIGH,
        },
        _ => bail!("Unknown priority level"),
    };

    writer.serialize(task)?;
    writer.flush()?;
    Ok(())
}


pub fn run_codex() -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("./tasks.csv")?;

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add { task, priority }) => add_task(task.clone(), priority, file)?,
        Some(Commands::Print {}) => print_tasks(file)?,
        Some(Commands::Remove { task }) => remove_task(task.clone(), file)?,
        None => {
            println!("Usage: codex [COMMAND], enter -h or --help for additional help");
        }
    }
    Ok(())
}

fn _load_config() {
    todo!()
}
