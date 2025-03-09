use axum::{Json, http::StatusCode};
use mongodb::bson::Document;

#[path = "../models/todo.rs"]
mod todo;

#[path = "../utils/mongo.rs"]
mod mongo;

pub async fn get_todos() -> (StatusCode, Json<Vec<Document>>) {
    let mongo: mongo::Mongo = mongo::Mongo::new("todos").await;
    let documents = mongo::Mongo::get_all_documents(mongo.get_db(),"todos").await;
    (StatusCode::OK, Json(documents))
}