pub mod err;

use std::{ error::Error, fs };

pub use crate::err::{ ParseErr, ReadErr };

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = fs::read_to_string(path).map_err(|e| {
            ReadErr {
                child_err: Box::new(e),
            }
        })?;
        let parse = json::parse(&content).map_err(|e| ParseErr::Malformed(Box::new(e)))?;
        let title = parse["title"].as_str().unwrap_or("Todo List").to_string();
        let mut tasks = Vec::new();
        for s in parse["tasks"].members() {
            let id = s["id"]
                .as_u32()
                .ok_or_else(|| {
                    ParseErr::Malformed(Box::<dyn Error>::from("Task missing 'id' or invalid type"))
                })?;
            let description = s["description"]
                .as_str()
                .ok_or_else(||
                    ParseErr::Malformed(Box::<dyn Error>::from("Task missing 'description'"))
                )?
                .to_string();
            let level = s["level"]
                .as_u32()
                .ok_or_else(|| {
                    ParseErr::Malformed(Box::<dyn Error>::from("Task missing 'level'"))
                })?;
            tasks.push(Task { id: id, description: description, level: level });
        }
        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }
        Ok(TodoList { title, tasks })
    }
}
