use axum::{
    routing::{get, post, put, delete},
    Router, response::Json, extract::{State, Path}, http::StatusCode
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Define the Todo struct
#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// Define the input struct for creating/updating todos
#[derive(Deserialize)]
struct CreateTodo {
    title: String,
    completed: bool,
}

// Shared application state
#[derive(Clone)]
struct AppState {
    todos: Arc<Mutex<HashMap<u32, Todo>>>,
}

#[tokio::main]
async fn main() {
    // Initialize the shared state with an empty HashMap
    let state = AppState {
        todos: Arc::new(Mutex::new(HashMap::new())),
    };

    // Build the Axum router with CRUD routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/todos", post(create_todo))
        .route("/todos", get(list_todos))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(state);

    // Start the server
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

// Root handler
async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({ "message": "Hello, World!" }))
}

// Create a new todo
async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let mut todos = state.todos.lock().unwrap();
    let id = (todos.len() as u32) + 1; // Simple ID generation
    let todo = Todo {
        id,
        title: payload.title,
        completed: payload.completed,
    };
    todos.insert(id, todo.clone());
    Ok((StatusCode::CREATED, Json(todo)))
}

// List all todos
async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap();
    Json(todos.values().cloned().collect())
}

// Get a single todo by ID
async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, StatusCode> {
    let todos = state.todos.lock().unwrap();
    match todos.get(&id) {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Update a todo by ID
async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let mut todos = state.todos.lock().unwrap();
    match todos.get_mut(&id) {
        Some(todo) => {
            todo.title = payload.title;
            todo.completed = payload.completed;
            Ok((StatusCode::OK, Json(todo.clone())))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Delete a todo by ID
async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    let mut todos = state.todos.lock().unwrap();
    match todos.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(StatusCode::NOT_FOUND),
    }
}
