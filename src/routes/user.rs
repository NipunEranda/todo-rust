use crate::{models::user::{LoginRequest, RegistrationRequest}, services, AppState};
use rocket::{http::Status, post, serde::json::Json, State};

// Route registry
#[post("/user/login", format = "json", data = "<login>")]
pub async fn login(state: &State<AppState>, login: Json<LoginRequest>) -> (Status, Json<String>) {
    services::user::login(state, login).await
}

#[post("/user/register", format = "json", data = "<registration>")]
pub async fn register(state: &State<AppState>, registration: Json<RegistrationRequest>) -> (Status, Json<String>) {
    services::user::create_user(state, registration).await
}