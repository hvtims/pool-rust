mod err;
pub use err::{ParseErr, ReadErr};

use std::{error::Error, fs};

#[derive(Debug, Eq, PartialEq, Clone)]
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
    pub fn get_todo(path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        let parsed = json::parse(&content).map_err(|e| ParseErr::Malformed(Box::new(e)))?;
        if parsed["tasks"].is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        let title = parsed["title"].as_str().ok_or(ParseErr::Empty)?.to_string();
        let mut tasks = Vec::new();

        for m in parsed["tasks"].members() {
            tasks.push(Task {
                id: m["id"].as_u32().unwrap(),
                description: m["description"].as_str().unwrap().to_string(),
                level: m["level"].as_u32().unwrap(),
            });
        }

        Ok(TodoList { title, tasks })
    }
}
