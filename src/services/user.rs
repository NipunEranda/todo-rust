use base64::{Engine, prelude::BASE64_STANDARD};
use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::{Collection, bson::doc};
use rocket::{State, http::Status, serde::json::Json};
use std::env;

use crate::{
    AppState,
    models::{
        claims::Claims,
        user::{LoginRequest, RegistrationRequest, User},
    },
};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub async fn create_user(
    state: &State<AppState>,
    registration: Json<RegistrationRequest>,
) -> (Status, Json<String>) {
    let mut user_id: String = String::from("0");
    let collection: Collection<User> = get_collection(state, "user").await;

    let salt: SaltString = SaltString::generate(&mut OsRng);
    let argon2: Argon2<'_> = Argon2::default();
    let password_hash: PasswordHash<'_> = argon2
        .hash_password(registration.password.as_bytes(), &salt)
        .ok()
        .unwrap();

    let user = User::new(
        registration.username.clone(),
        BASE64_STANDARD.encode(password_hash.to_string()),
    );

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(user).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        user_id = inserted_id.to_hex();
    }

    (Status::Ok, Json(user_id))
}

pub async fn login(state: &State<AppState>, login: Json<LoginRequest>) -> (Status, Json<String>) {
    let collection = get_collection(state, "user").await;

    let user: Option<User> = collection
        .find_one(doc! {"username": login.username.clone()})
        .await
        .ok()
        .unwrap();

    if !user.is_some() {
        return (Status::Unauthorized, Json("".to_string()));
    }
    let user = &user.unwrap();

    let decoded_password = String::from_utf8(BASE64_STANDARD.decode(user.password.clone()).ok().unwrap()).ok().unwrap();
    let pwd_hash: Option<PasswordHash<'_>> = PasswordHash::new(&decoded_password).ok();

    if !pwd_hash.is_some() {
        return (Status::Unauthorized, Json("".to_string()));
    }

    if Argon2::default()
        .verify_password(login.password.as_bytes(), &pwd_hash.unwrap())
        .is_ok()
    {
        let claims = Claims::new(user._id.to_hex(), user.username.clone());
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(env::var("JWT_KEY").ok().unwrap().as_bytes()),
        );
        (Status::Ok, Json(token.ok().unwrap().to_string()))
    } else {
        (Status::Unauthorized, Json("".to_string()))
    }
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<User> {
    let client: rocket::tokio::sync::MutexGuard<'_, mongodb::Client> =
        state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("todo");
    db.collection::<User>(collection)
}
