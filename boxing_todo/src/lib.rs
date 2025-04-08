mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::{fs::File, io::Read};
extern crate json;

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
        let mut file = File::open(path).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            })
        })?;
        
        let mut s = String::new();
        file.read_to_string(&mut s).map_err(|e| {
            Box::new(ReadErr {
                child_err: Box::new(e),
            })
        })?;
        
        if s.trim().is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }
        
        let parsed_json = match json::parse(&s) {
            Ok(json) => json,
            Err(e) => return Err(Box::new(ParseErr::Malformed(Box::new(e)))),
        };
        
        let title = match parsed_json["title"].as_str() {
            Some(t) => t.to_string(),
            None => return Err(Box::new(ParseErr::Empty)),
        };

        let mut tasks: Vec<Task> = Vec::new();
        if parsed_json["tasks"].len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        for i in 0..parsed_json["tasks"].len() {
            let id = match parsed_json["tasks"][i]["id"].as_u32() {
                Some(id) => id,
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            let description = match parsed_json["tasks"][i]["description"].as_str() {
                Some(desc) => desc.to_string(),
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            let level = match parsed_json["tasks"][i]["level"].as_u32() {
                Some(lvl) => lvl,
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            tasks.push(Task {
                id,
                description,
                level,
            });
        }
        
        Ok(TodoList { title, tasks })
    }
}