use mongodb::bson::doc;
use rocket::{State, delete, get, post, put, serde::json::Json};

use crate::{
    AppState,
    models::todo::{TodoRequest, TodoResponse},
    services,
};

// Route registry
#[get("/")]
pub async fn index(state: &State<AppState>) -> Json<Vec<TodoResponse>> {
    let todos: Vec<TodoResponse> = services::todo::get_todos(state).await;
    Json(todos)
}

#[post("/", format = "json", data = "<todo>")]
pub async fn create(state: &State<AppState>, todo: Json<TodoRequest>) -> Json<String> {
    Json(services::todo::create_todo(state, todo).await)
}

#[put("/<id>", format = "json", data = "<todo>")]
pub async fn update(state: &State<AppState>, id: &str, todo: Json<TodoRequest>) -> Json<bool> {
    Json(services::todo::update_todo(state, String::from(id), todo).await)
}

#[delete("/perm/<id>")]
pub async fn delete(state: &State<AppState>, id: &str) -> Json<bool> {
    Json(services::todo::delete_todo_perm(state, String::from(id)).await)
}
