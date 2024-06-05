pub use std::error::Error;

pub use json::{parse, stringify};
use json::JsonValue;

use crate::err::{ParseErr, ReadErr};

mod err;

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
    fn parse(j: JsonValue) -> Option<TodoList> {
        let mut tasks: Vec<Task> = Vec::new();
        match j {
            JsonValue::Object(m) => {
                let title = m.get("title")?.as_str()?.to_string();
                if let Some(JsonValue::Array(n)) = m.get("tasks") {
                    for i in n {
                        if let JsonValue::Object(m) = i {
                            tasks.push(Task {
                                id: m.get("id")?.as_u32()?,
                                description: m.get("description")?.as_str()?.to_string(),
                                level: m.get("level")?.as_u32()?,
                            })
                        }
                    }
                    return Some(TodoList {
                        title: title,
                        tasks,
                    });
                }
            }
            _ => (),
        }
        return None;
    }
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file = match std::fs::read_to_string(path) {
            Err(e) => {
                return Err(Box::new(ReadErr {
                    child_err: Box::new(e),
                }))
            }
            Ok(s) => s,
        };
        let todo_list = parse(&file);
        match todo_list {
            Err(er) => Err(Box::new(ParseErr::Malformed(Box::new(er)))),
            Ok(list) => match TodoList::parse(list) {
                Some(t) => {
                    if t.tasks.len() != 0 {
                        Ok(t)
                    } else {
                        Err(Box::new(ParseErr::Empty))
                    }
                }
                None => panic!("tets"),
            },
        }
    }
}