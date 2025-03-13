use mongodb::{Collection, bson::Document};
use rocket::{State, serde::json::Json};

use crate::{
    AppState,
    models::user::{RegistrationRequest, User},
    utils,
};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub async fn create_user(
    state: &State<AppState>,
    registration: Json<RegistrationRequest>,
) -> String {
    let mut user_id: String = String::from("0");
    let collection: Collection<User> = get_collection(state, "user").await;

    let salt = SaltString::generate(&mut OsRng);
    let argon2: Argon2<'_> = Argon2::default();
    let password_hash: PasswordHash<'_> = argon2.hash_password(registration.password.as_bytes(), &salt).ok().unwrap();

    let user = User::new(registration.username.clone(), password_hash.to_string());

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(user).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        user_id = inserted_id.to_hex();
    }

    user_id
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<User> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("todo");
    db.collection::<User>(collection)
}
