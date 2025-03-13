use mongodb::{Collection, bson::doc};
use rocket::{State, serde::json::Json};

use crate::{
    AppState,
    models::user::{LoginRequest, RegistrationRequest, User},
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
    let password_hash: PasswordHash<'_> = argon2
        .hash_password(registration.password.as_bytes(), &salt)
        .ok()
        .unwrap();

    let user = User::new(registration.username.clone(), password_hash.to_string());

    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(user).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        user_id = inserted_id.to_hex();
    }

    user_id
}

pub async fn login(state: &State<AppState>, login: Json<LoginRequest>) -> String {
    let token = String::from("");

    let collection = get_collection(state, "user").await;

    let user = collection
        .find_one(doc! {"username": login.username.clone()})
        .await
        .ok()
        .unwrap();

    if !user.is_some() {
        return "".to_string();
    }
    let user: User = user.unwrap();
    let pwd_hash = PasswordHash::new(&user.password).ok();

    if !pwd_hash.is_some() {
        return "".to_string();
    }

    let pwd_hash = pwd_hash.unwrap();

    println!(
        "{:?}",
        Argon2::default()
            .verify_password(login.password.as_bytes(), &pwd_hash)
            .is_ok()
    );

    "success".to_string()
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<User> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("todo");
    db.collection::<User>(collection)
}
