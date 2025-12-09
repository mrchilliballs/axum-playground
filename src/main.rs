use std::sync::{
    Arc, Mutex,
    atomic::{AtomicI32, Ordering},
};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let counter = Router::new()
        .route("/increment", post(increment))
        .route("/", get(count))
        .with_state(Arc::new(AtomicI32::new(0)));
    let app = Router::new()
        .route("/", get(root))
        .route("/add", post(add))
        .nest("/counter", counter);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

#[derive(Debug, Deserialize)]
struct Add {
    left: i32,
    right: i32,
}

async fn add(Json(Add { left, right }): Json<Add>) -> Json<Value> {
    Json(json!({ "result": left + right }))
}

#[derive(Debug, Deserialize)]
struct Increment {
    val: i32,
}
async fn increment(
    State(count): State<Arc<AtomicI32>>,
    Json(Increment { val }): Json<Increment>,
) -> StatusCode {
    count.fetch_add(val, Ordering::Relaxed);
    StatusCode::OK
}

async fn count(State(count): State<Arc<AtomicI32>>) -> Json<Value> {
    Json(json!( { "count": count.load(Ordering::Relaxed) }))
}
