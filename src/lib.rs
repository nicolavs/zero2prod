use axum::http::StatusCode;
use axum::{routing::get, Router};

use axum::extract::Path;

use tokio::net::TcpListener;

async fn greet(Path(user_name): Path<String>) -> String {
    format!("Hello {}!", user_name)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

fn app() -> Router {
    Router::new()
        .route("/{name}", get(greet))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
}

pub async fn run(listener: TcpListener) {
    axum::serve(listener, app()).await.unwrap();
}
