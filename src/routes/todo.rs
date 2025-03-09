use axum::{Json, http::StatusCode};

#[path = "../models/todo.rs"]
mod todo;

pub async fn get_todos() -> (StatusCode, Json<todo::Todo>) {
    let todo = todo::Todo::new(1, String::from("Buy milk"));

    (StatusCode::CREATED, Json(todo))
}
