use anyhow::{anyhow, bail, Result};
use clap::{arg, crate_authors, crate_description, crate_name, crate_version, Command};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{create_dir_all, File, OpenOptions},
    path::{Path, PathBuf},
};

fn cli() -> Command {
    Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
            .about("Adds a task")
            .arg(
                arg!(-p --priority <priority> "Priority order of the task")
                .required(false)
                .default_value("low")
                )
            .arg(arg!(<task> "The task to be added"))
            .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("list")
            .about("Lists the tasks present. It defaults to showing incompleted tasks")
            .arg(
                arg!(-p --priority <priority> "Priority of the tasks to list. If none is given, it defaults to listing all the tasks")
                .required(false)
                .default_value("any")
                )
        )
        .subcommand(
            Command::new("flip")
            .about("Flips the state of the task")
            .arg(arg!(<number> "Number in the list of the task to be flipped"))
            .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("remove")
            .about("Removes a task forever")
            .arg(arg!(<number> "Number in the list of the task to be removed"))
            .arg_required_else_help(true)
            )
}

pub fn run() -> Result<()> {
    let tasks_file_path = create_tasks_file()?;
    let tasks_file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(tasks_file_path)?;

    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let task = sub_matches
                .get_one::<String>("task")
                .expect("requierd argument");
            let priority = sub_matches
                .get_one::<String>("priority")
                .expect("required argument");

            add_task(task.to_string(), priority, tasks_file)?
        }
        Some(("list", sub_matches)) => {
            let priority = sub_matches
                .get_one::<String>("priority")
                .expect("required argument");

            list_tasks(priority, tasks_file)?;
        }
        Some(("flip", sub_matches)) => {
            let number = sub_matches
                .get_one::<String>("number")
                .expect("Required argument")
                .parse::<usize>()?;

            flip_task(number, tasks_file)?;
        },
        Some(("remove", sub_matches)) => {
            let number = sub_matches
                .get_one::<String>("number")
                .expect("Required argument")
                .parse::<usize>()?;

            remove_task(number, tasks_file)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority {
    HIGH,
    MEDIUM,
    LOW,
}

// impl core::fmt::Display for Priority {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             Priority::HIGH => write!(f, "High"),
//             Priority::MEDIUM => write!(f, "Medium"),
//             Priority::LOW => write!(f, "Low"),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task: String,
    priority: Priority,
    completed: bool,
}

impl core::fmt::Display for Task {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let state = match self.completed {
            true => "X",
            false => " ",
        };

        let task = match self.priority {
            Priority::LOW => format!(
                "[{}] - {}",
                state.bold(),
                self.task.to_string().green().bold(),
            ),
            Priority::MEDIUM => format!(
                "[{}] - {}",
                state.bold(),
                self.task.to_string().yellow().bold(),
            ),
            Priority::HIGH => format!(
                "[{}] - {}",
                state.bold(),
                self.task.to_string().red().bold(),
            ),
        };
        write!(f, "{task}")
    }
}

fn read_tasks(file: File) -> Result<Vec<Task>> {
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

fn remove_task(needle: usize, file: File) -> Result<()> {
    let mut tasks = read_tasks(file.try_clone()?)?;
    tasks.remove(needle - 1);
    file.set_len(0)?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    for task in tasks {
        writer.serialize(task)?;
    }

    writer.flush()?;
    Ok(())
}

fn flip_task(needle: usize, file: File) -> Result<()> {
    let mut tasks = read_tasks(file.try_clone()?)?;

    if let Some(task) = tasks.get_mut(needle - 1) {
        task.completed = !task.completed;

        file.set_len(0)?;

        let mut writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);

        for task in tasks {
            writer.serialize(task)?;
        }

        writer.flush()?;

        Ok(())
    } else {
        Err(anyhow!("Task not found"))
    }
}

fn print_task(mode: Option<Priority>, file: File) -> Result<()> {
    match mode {
        Some(needle) => read_tasks(file)?
            .into_iter()
            .filter(|task| task.priority == needle)
            .for_each(|task| println!("{task}")),
        None => read_tasks(file)?
            .into_iter()
            .enumerate()
            .for_each(|(idx, task)| println!("     {}. {task}", idx + 1)),
    }
    Ok(())
}

fn list_tasks(priority: &str, file: File) -> Result<()> {
    match priority {
        "low" | "l" => print_task(Some(Priority::LOW), file)?,
        "medium" | "m" => print_task(Some(Priority::MEDIUM), file)?,
        "high" | "h" => print_task(Some(Priority::HIGH), file)?,
        "any" | "a" => print_task(None, file)?,
        _ => bail!("Unkown priority level"),
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
            completed: false,
        },
        "medium" | "m" => Task {
            task,
            priority: Priority::MEDIUM,
            completed: false,
        },
        "high" | "h" => Task {
            task,
            priority: Priority::HIGH,
            completed: false,
        },
        _ => bail!("Unknown priority level"),
    };

    writer.serialize(task)?;
    writer.flush()?;
    Ok(())
}

fn create_tasks_file() -> Result<PathBuf> {
    let xdg_data_home_default = env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(|home| Path::new(&home).join(".local/share"))
                .unwrap()
        });

    let dir_file_path = xdg_data_home_default.join("codex");
    create_dir_all(&dir_file_path)?;
    Ok(dir_file_path.join("tasks.csv"))
}

fn _load_config() {
    // XDG_CONFIG_HOME
    todo!()
}
