use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
    result::Result,
};

fn get_storage_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("notemanager");
    let _ = create_dir_all(&path);
    path.push("tasks.json");

    if !path.exists() && Path::new("tasks.json").exists() {
        let _ = std::fs::copy("tasks.json", &path);
    }

    path
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

pub enum Command {
    Add(String),
    Done(Vec<usize>),
    Delete(Vec<usize>),
    List,
    Quit,
}

#[derive(Serialize, Deserialize)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
}

pub fn validate_user_input(input: &str) -> Result<Command, String> {
    let trimmed = input.trim();
    match trimmed {
        "List" => Ok(Command::List),
        "Quit" => Ok(Command::Quit),
        _ => match trimmed.split_once(" ") {
            Some(("Add", rest)) => Ok(Command::Add(rest.to_string())),
            Some(("Done", rest)) => {
                let mut ids = Vec::new();
                for part in rest.split_whitespace() {
                    match part.parse::<usize>() {
                        Ok(usize) => ids.push(usize),
                        Err(_) => return Err("This command needs an integer.".to_string()),
                    }
                }
                Ok(Command::Done(ids))
            },
            Some(("Delete", rest)) => {
                let mut ids = Vec::new();
                for part in rest.split_whitespace(){
                    match part.parse::<usize>() {
                        Ok(usize) => ids.push(usize),
                        Err(_) => return Err("This command needs an integer.".to_string())
                    }
                }
                Ok(Command::Delete(ids))
            }
            _ => Err("Could't recognize a command".to_string()),
        },
    }
}

impl TaskManager {
    fn generate_id(&self) -> usize {
        let mut id = 1;
        while self.tasks.iter().any(|task| task.id == id) {
            id += 1
        }
        id
    }
    fn save(&self) -> Result<(), String> {
        let path = get_storage_path();
        match to_string_pretty(&self) {
            Ok(data) => match write(&path, data) {
                Ok(_) => Ok(()),
                Err(error) => Err(format!("Error saving: {}", error)),
            },
            Err(error) => Err(format!("Error formatting save: {}", error)),
        }
    }

    pub fn add(&mut self, title: String) {
        self.tasks.push(Task {
            id: self.generate_id(),
            title,
            completed: false,
        });
        self.save().expect("Failed to save tasks");
    }
    pub fn list(&self) {
        for task in &self.tasks {
            println!(
                "Task: {}, Title: {}, Completed: {}",
                task.id, task.title, task.completed
            )
        }
    }
    pub fn done(&mut self, ids: Vec<usize>) {
        for task in &mut self.tasks {
            if ids.contains(&task.id) {
                task.completed = true;
            }
        };
        self.save().expect("Failed to save tasks");
    }
    pub fn toggle(&mut self, id: usize) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = !task.completed;
            }
        }
        self.save().expect("Failed to save tasks");
    }
    pub fn delete(&mut self, ids: Vec<usize>){
        self.tasks.retain(|task| !ids.contains(&task.id));
        self.save().expect("Failed to save tasks");
    }
    pub fn new() -> Result<Self, String> {
        let path = get_storage_path();
        match read_to_string(&path) {
            Ok(content) => match from_str(&content) {
                Ok(content) => Ok(content),
                Err(error) => Err(format!("Error loading: {}", error)),
            },
            Err(_) => Ok(TaskManager { tasks: Vec::new() }),
        }
    }
}

