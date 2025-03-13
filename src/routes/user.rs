use crate::{models::user::{LoginRequest, RegistrationRequest}, services, AppState};
use rocket::{State, post, serde::json::Json};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

// Route registry
#[post("/user/login", format = "json", data = "<login>")]
pub async fn login(state: &State<AppState>, login: Json<LoginRequest>) -> Json<String> {

    // let salt = SaltString::generate(&mut OsRng);
    // let password_hash: PasswordHash<'_> = argon2.hash_password(login_request.password.as_bytes(), &salt).ok().unwrap();
    // Argon2::default().verify_password(login_request.password.as_bytes(), &password_hash).is_ok()

    Json(services::user::login(state, login).await)
}

#[post("/user/register", format = "json", data = "<registration>")]
pub async fn register(state: &State<AppState>, registration: Json<RegistrationRequest>) -> Json<String> {
    Json(services::user::create_user(state, registration).await)
}