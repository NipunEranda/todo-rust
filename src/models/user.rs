use std::time::SystemTime;

use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub password: String,
    pub created: DateTime,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub created: DateTime,
    pub is_active: bool,
}

impl TryFrom<RegistrationRequest> for User {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: RegistrationRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = chrono::Utc::now().into();

        Ok(Self {
            username: item.username,
            password: item.password.to_string(),
            _id: ObjectId::new(),
            created: DateTime::from(chrono_datetime),
            is_active: true,
        })
    }
}

impl TryFrom<LoginRequest> for User {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: LoginRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = chrono::Utc::now().into();

        Ok(Self {
            _id: ObjectId::new(),
            username: item.username,
            password: item.password.to_string(),
            created: DateTime::from(chrono_datetime),
            is_active: true,
        })
    }
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        let chrono_datetime: SystemTime = chrono::Utc::now().into();
        User {
            _id: ObjectId::new(),
            username,
            password,
            created: DateTime::from(chrono_datetime),
            is_active: true,
        }
    }
}
