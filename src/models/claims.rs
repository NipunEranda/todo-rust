use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    sub: String,
    username: String,
}

impl Claims {
    pub fn new(sub: String, username: String) -> Self {
        Claims {
            exp: Utc::now()
                .checked_add_signed(chrono::Duration::seconds(86400))
                .expect("Valid timestamp")
                .timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            sub,
            username,
        }
    }

    pub fn _get_username(&self) -> String {
        return String::from(&self.username);
    }

    pub fn _get_id(&self) -> String {
        return String::from(&self.sub);
    }
}