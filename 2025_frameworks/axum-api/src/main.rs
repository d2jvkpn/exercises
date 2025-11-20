use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use axum::{
    Router,
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode, // Request
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type AppState = Arc<RwLock<HashMap<Uuid, User>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    age: u8,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
    age: u8,
}

#[tokio::main]
async fn main() {
    let state: AppState = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        //.route("/health", get(|| async { "ok" }))
        .with_state(state)
        .fallback(not_found);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Rust HTTP API Server is running.\n"
    // chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
}

async fn health_check() -> &'static str {
    "OK\n"
}

async fn not_found(req: Request<Body>) -> impl IntoResponse {
    let body = serde_json::json!({
        "code": "route_not_found", // 404
        "msg": "Not route",
        "data": { "api": format!("{}@{}", req.method(), req.uri().path()) },
    });

    (StatusCode::NOT_FOUND, Json(body))
}

async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.read().unwrap();
    let users_vec: Vec<User> = users.values().cloned().collect();

    Json(users_vec)
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let users = state.read().unwrap();

    if let Some(user) = users.get(&id) {
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let id = Uuid::new_v4();

    let user = User { id, name: payload.name, email: payload.email, age: payload.age };

    let mut users = state.write().unwrap();
    users.insert(id, user.clone());

    Ok(Json(user))
}

async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.write().unwrap();

    if let Some(user) = users.get_mut(&id) {
        user.name = payload.name;
        user.email = payload.email;
        user.age = payload.age;

        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut users = state.write().unwrap();

    if users.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
