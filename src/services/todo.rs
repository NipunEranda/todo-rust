use crate::{
    AppState,
    models::todo::{Todo, TodoRequest, TodoResponse},
};
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use rocket::{futures::TryStreamExt, http::Status, serde::json::Json, State};

pub async fn get_todos(state: &State<AppState>) -> (Status, Json<Vec<TodoResponse>>) {
    let mut todos: Vec<TodoResponse> = Vec::new();
    let collection: Collection<Todo> = get_collection(state, "todo").await;
    let result = collection.find(doc! {}).await;
    let cursor = match result {
        Ok(cursor) => cursor,
        Err(_) => return (Status::BadRequest, Json(vec![])),
    };

    cursor
        .try_collect()
        .await
        .unwrap_or(vec![])
        .iter()
        .for_each(|todo| {
            todos.push(TodoResponse::new(
                todo._id.clone().to_hex(),
                todo.name.clone(),
                todo.created.clone(),
                todo.completed.clone(),
            ));
        });

    (Status::Ok, Json(todos))
}

pub async fn create_todo(state: &State<AppState>, todo: Json<TodoRequest>) -> (Status, Json<String>) {
    let mut todo_id: String = String::from("0");
    let collection: Collection<Todo> = get_collection(state, "todo").await;
    let todo: Todo = Todo::try_from(todo.into_inner()).unwrap();
    let result: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        collection.insert_one(todo).await;
    if let Some(inserted_id) = result.unwrap().inserted_id.as_object_id() {
        todo_id = inserted_id.to_hex();
    }
    (Status::Ok, Json(todo_id))
}

pub async fn update_todo(state: &State<AppState>, id: String, todo: Json<TodoRequest>) -> (Status, Json<bool>) {
    let collection: Collection<Todo> = get_collection(state, "todo").await;
    let todo: TodoRequest = TodoRequest::try_from(todo.into_inner()).unwrap();

    if !ObjectId::parse_str(&id).is_ok() {
        return (Status::NotModified, Json(false));
    }

    let todo_id = ObjectId::parse_str(id).ok().unwrap_or_default();

    let existing_todo_result = collection.find_one(doc! {"_id": todo_id}).await;

    if existing_todo_result.ok().unwrap().is_none() {
        return (Status::NotFound, Json(false));
    }

    collection
        .update_one(
            doc! {"_id": todo_id},
            doc! { "$set": doc! {"name": todo.name, "completed": todo.completed} },
        )
        .await
        .ok()
        .unwrap();

    (Status::Ok, Json(true))
}

pub async fn delete_todo_perm(state: &State<AppState>, id: String) -> (Status, Json<bool>) {
    let collection: Collection<Todo> = get_collection(state, "todo").await;

    if !ObjectId::parse_str(&id).is_ok() {
        return (Status::NotModified, Json(false));
    }

    let todo_id = ObjectId::parse_str(id).ok().unwrap_or_default();

    let existing_todo_result = collection.find_one(doc! {"_id": todo_id}).await;

    if existing_todo_result.ok().unwrap().is_none() {
        return (Status::NotFound, Json(false));
    }

    collection
        .delete_one(doc! {"_id": todo_id})
        .await
        .ok()
        .unwrap();

    (Status::Ok, Json(true))
}

async fn get_collection(state: &State<AppState>, collection: &str) -> Collection<Todo> {
    let client = state.mongo_client.lock().await;
    let db: mongodb::Database = client.database("todo");
    db.collection::<Todo>(collection)
}
