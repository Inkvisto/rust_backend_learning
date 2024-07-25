use axum::{
    routing::{get, post, get_service},
    http::StatusCode,
    response::IntoResponse,
    Json, Router};

use axum::extract::Path;
use tower_http::services::ServeFile;

use serde::{Deserialize, Serialize};
use serde_json::json;

use std::io;

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)]
struct User {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user))
        .route("/hello/:name", get(json_hello))
        .route("/static", get_service(ServeFile::new("static/hello.html")));
          
     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(payload): Json<CreateUser>,) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username
    };

    (StatusCode::CREATED, Json(user))
}


async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
    let greeting = name.as_str();
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({"message": hello + greeting })))
}
