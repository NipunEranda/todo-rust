use mongodb::Collection;
use rocket::State;

use crate::{AppState, models::todo::Todo};

pub async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<Todo> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("todo");
    db.collection::<Todo>(collection)
}