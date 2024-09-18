use std::{cmp::max, fs, path::Path};

use chrono::Local;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[arg()]
    Add {
        #[arg()]
        message: String,
    },
    List(ListCommands),
    Update {
        #[arg()]
        id: i64,
        #[arg()]
        message: String,
    },
    Delete {
        #[arg()]
        id: i64,
    },
    MarkInProgress {
        #[arg()]
        id: i64,
    },
    MarkDone {
        #[arg()]
        id: i64,
    },
}

#[derive(Parser)]
struct ListCommands {
    #[command(subcommand)]
    command: Option<ListEnumCommands>,
}

#[derive(Subcommand)]
enum ListEnumCommands {
    Done,
    Todo,
    InProgress,
}

#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    id: i64,
    description: String,
    status: String,
    created_at: String,
    updated_at: String,
}

// helper function
fn save_to_file(todo_data: Vec<Todo>) {
    let path = Path::new("./todo-db.json");

    // convert todo_data to string
    let file_string = match serde_json::to_string(&todo_data) {
        Ok(value) => value,
        Err(_) => panic!("Unable to convert todo data into string. {:?}", todo_data),
    };

    // save the file to path
    if let Err(_) = fs::write(path, file_string) {
        panic!("Unable to save file. path: {:?}", path);
    }
}

fn get_data_from_file() -> Vec<Todo> {
    let path = Path::new("./todo-db.json");

    let file_string = match fs::read_to_string(path) {
        Ok(val) => val,
        Err(_) => {
            // create a new file called todo-db.json
            panic!("Unable to read todo-db.json")
        }
    };

    let todo_data = match serde_json::from_str(&file_string) {
        Ok(value) => value,
        Err(_) => panic!("Unable to deserialize data from file. path: {:?}", path),
    };
    todo_data
}

fn add_todo(message: &String) {
    let path = Path::new("./todo-db.json");
    if !path.exists() {
        fs::File::create(path).unwrap();
    }

    let mut todo_datas = get_data_from_file();

    // get the last id
    let todo_id = (todo_datas.iter().map(|todo| todo.id).reduce(max).unwrap() + 1) as i64;

    let current_time = Local::now().to_string();

    let todo_struct = Todo {
        id: todo_id,
        description: message.to_owned(),
        status: "todo".to_string(),
        created_at: current_time.clone(),
        updated_at: current_time,
    };

    todo_datas.push(todo_struct);

    save_to_file(todo_datas);

    println!("Task added successfully: (ID: {})", todo_id);
}

fn update_todo(id: &i64, message: &String) {
    let todo_datas = get_data_from_file();

    let todo_datas: Vec<Todo> = todo_datas
        .into_iter()
        .map(|mut value| {
            if value.id == id.to_owned() {
                // update message here
                value.description = message.to_owned();
            }
            return value;
        })
        .collect::<Vec<Todo>>();

    save_to_file(todo_datas);
    println!("Task updated successfully: (ID: {})", id);
}

fn delete_todo(id: &i64) {
    let todo_datas = get_data_from_file();

    let todo_datas: Vec<Todo> = todo_datas
        .into_iter()
        .filter(|value| {
            if value.id != id.to_owned() {
                return true;
            } else {
                return false;
            }
        })
        .collect::<Vec<Todo>>();

    save_to_file(todo_datas);
    println!("Task deleted successfully: (ID: {})", id);
}

fn print_todo(todo: &Todo) {
    println!(
        "Todo ID: {}\nTodo Description: {}\nTodo Status: {}\nTodo Created At: {}\nTodo Updated At: {}\n----------",
        todo.id, todo.description, todo.status,todo.created_at,todo.updated_at
    );
}

fn list_todo(subcommands: &Option<ListEnumCommands>) {
    let todo_datas = get_data_from_file();
    if let Some(command) = subcommands {
        match command {
            ListEnumCommands::Done => todo_datas.iter().for_each(|value| {
                if value.status == "done" {
                    print_todo(value);
                }
            }),
            ListEnumCommands::InProgress => todo_datas.iter().for_each(|value| {
                if value.status == "in-progress" {
                    print_todo(value);
                }
            }),
            ListEnumCommands::Todo => todo_datas.iter().for_each(|value| {
                if value.status == "todo" {
                    print_todo(value);
                }
            }),
        }
    }
}

fn mark_in_progress_todo(id: &i64) {
    let todo_datas = get_data_from_file();

    let todo_datas: Vec<Todo> = todo_datas
        .into_iter()
        .map(|mut value| {
            if value.id == id.to_owned() {
                // update message here
                value.status = "in-progress".to_string();
            }
            return value;
        })
        .collect::<Vec<Todo>>();

    save_to_file(todo_datas);
    println!("Task marked in-progress: (ID: {})", id);
}

fn mark_done_todo(id: &i64) {
    let todo_datas = get_data_from_file();

    let todo_datas: Vec<Todo> = todo_datas
        .into_iter()
        .map(|mut value| {
            if value.id == id.to_owned() {
                // update message here
                value.status = "done".to_string();
            }
            return value;
        })
        .collect::<Vec<Todo>>();

    save_to_file(todo_datas);
    println!("Task done: (ID: {})", id);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { message }) => add_todo(message),
        Some(Commands::Update { id, message }) => update_todo(id, message),
        Some(Commands::Delete { id }) => delete_todo(id),
        Some(Commands::List(ListCommands { command })) => list_todo(command),
        Some(Commands::MarkInProgress { id }) => mark_in_progress_todo(id),
        Some(Commands::MarkDone { id }) => mark_done_todo(id),
        _ => panic!("Woops"),
    }
}
