mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

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
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let content = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }))?;
        
        let parse = json::parse(&content)
            .map_err(|e| ParseErr::Malformed(Box::new(e)))?;
        
        let tasks: Vec<Task> = parse["tasks"]
            .members()
            .map(|m| Task {
                id: m["id"].as_u32().unwrap(),
                description: m["description"].as_str().unwrap().to_string(),
                level: m["level"].as_u32().unwrap(),
            })
            .collect();
        
        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList {
            title: parse["title"].as_str().unwrap().to_string(),
            tasks,
        })
    }
}
