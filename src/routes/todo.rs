use mongodb::bson::doc;
use rocket::{delete, get, http::Status, post, put, serde::json::Json, State};

use crate::{
    models::todo::{TodoRequest, TodoResponse}, services::todo, utils::request_guard::HeaderGuard, AppState
};

// Route registry
#[get("/todo")]
pub async fn index(_guard: HeaderGuard, state: &State<AppState>) -> (Status, Json<Vec<TodoResponse>>) {
    todo::get_todos(state).await
}

#[post("/todo", format = "json", data = "<todo>")]
pub async fn create(_guard: HeaderGuard, state: &State<AppState>, todo: Json<TodoRequest>) -> (Status, Json<String>) {
    todo::create_todo(state, todo).await
}

#[put("/todo/<id>", format = "json", data = "<todo>")]
pub async fn update(_guard: HeaderGuard, state: &State<AppState>, id: &str, todo: Json<TodoRequest>) -> (Status, Json<bool>) {
    todo::update_todo(state, String::from(id), todo).await
}

#[delete("/todo/perm/<id>")]
pub async fn delete(_guard: HeaderGuard, state: &State<AppState>, id: &str) -> (Status, Json<bool>) {
    todo::delete_todo_perm(state, String::from(id)).await
}
