use mongodb::bson::doc;
use rocket::{State, delete, get, post, put, serde::json::Json};

use crate::{
    AppState,
    models::todo::{TodoRequest, TodoResponse},
    services,
};

// Route registry
#[get("/todo")]
pub async fn index(state: &State<AppState>) -> Json<Vec<TodoResponse>> {
    let todos: Vec<TodoResponse> = services::todo::get_todos(state).await;
    Json(todos)
}

#[post("/todo", format = "json", data = "<todo>")]
pub async fn create(state: &State<AppState>, todo: Json<TodoRequest>) -> Json<String> {
    Json(services::todo::create_todo(state, todo).await)
}

#[put("/todo/<id>", format = "json", data = "<todo>")]
pub async fn update(state: &State<AppState>, id: &str, todo: Json<TodoRequest>) -> Json<bool> {
    Json(services::todo::update_todo(state, String::from(id), todo).await)
}

#[delete("/todo/perm/<id>")]
pub async fn delete(state: &State<AppState>, id: &str) -> Json<bool> {
    Json(services::todo::delete_todo_perm(state, String::from(id)).await)
}
