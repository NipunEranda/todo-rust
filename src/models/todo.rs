use std::time::SystemTime;

use mongodb::bson::{self, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub _id: ObjectId,
    pub name: String,
    pub created: DateTime,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoRequest {
    pub name: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoResponse {
    pub id: String,
    pub name: String,
    pub created: DateTime,
    pub completed: bool,
}

impl TryFrom<TodoRequest> for Todo {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: TodoRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = chrono::Utc::now().into();

        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            created: DateTime::from(chrono_datetime),
            completed: false,
        })
    }
}

impl TodoResponse {
    pub fn new(
        id: String,
        name: String,
        created: bson::DateTime,
        completed: bool,
    ) -> Self {
        TodoResponse {
            id,
            name,
            created,
            completed,
        }
    }
}
