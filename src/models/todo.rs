// use serde::{Deserialize, Serialize};

use serde::Serialize;

// #[derive(Deserialize)]
// pub struct CreateTodo {
//     name: String,
// }

// impl CreateTodo {
//     pub fn new(name: String) -> Self {
//         Self { name }
//     }
// }

#[derive(Serialize)]
pub struct Todo {
    name: String,
    completed: bool,
}

impl Todo {
    pub fn new(name: String, completed: bool) -> Self {
        Self { name, completed }
    }
}