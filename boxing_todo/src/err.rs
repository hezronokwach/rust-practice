mod err;

use std::{error::Error, fs};
use err::{ParseErr, ReadErr};
use json;

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
        // Read the file
        let content = fs::read_to_string(path).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            }) as Box<dyn Error>
        })?;

        // Parse the JSON
        let parsed = json::parse(&content).map_err(|e| {
            Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>
        })?;

        // Extract title
        let title = parsed["title"].as_str().unwrap_or("").to_string();

        // Extract tasks
        let tasks_json = &parsed["tasks"];
        
        // Check if tasks array is empty
        if tasks_json.is_array() && tasks_json.members().count() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        // Parse tasks
        let mut tasks = Vec::new();
        for task in tasks_json.members() {
            let id = task["id"].as_u32().unwrap_or(0);
            let description = task["description"].as_str().unwrap_or("").to_string();
            let level = task["level"].as_u32().unwrap_or(0);

            tasks.push(Task {
                id,
                description,
                level,
            });
        }

        Ok(TodoList { title, tasks })
    }
}
