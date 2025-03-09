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
    id: i64,
    name: String,
}

impl Todo {
    pub fn new(id: i64, name: String) -> Self {
        Self { id, name }
    }
}