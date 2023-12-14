mod err;

#[warn(unused_imports)]
use err::{ParseErr, ReadErr};
use json::parse;
use std::error::Error;
use std::fs::File;
use std::io::Read;

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
        let mut file = File::open(path).map_err(|err| ReadErr {
            child_err: Box::new(err),
        })?;

        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|err| ReadErr {
            child_err: Box::new(err),
        })?;

        let parsed_json: json::JsonValue = parse(&content).map_err(|err| ParseErr::Malformed(Box::new(err)))?;

        let title = parsed_json["title"].as_str().unwrap_or("Untitled").to_string();

        let tasks: Vec<_> = parsed_json["tasks"]
            .members()
            .map(|task| Task {
                id: task["id"].as_u32().unwrap_or(0),
                description: task["description"].as_str().unwrap_or("").to_string(),
                level: task["level"].as_u32().unwrap_or(0),
            })
            .collect();

        if tasks.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }

        Ok(TodoList { title, tasks })
    }
}
