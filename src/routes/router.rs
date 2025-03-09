use axum::{
    Router,
    routing::get,
};

#[path = "./todo.rs"]
mod todo;

// Route registry
pub fn router() -> Router {
    Router::new()
    .route("/", get(todo::get_todos))
}