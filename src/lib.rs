use axum::http::StatusCode;
use axum::{routing::get, Router};

use axum::extract::Path;

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

pub fn run() -> impl std::future::Future<Output = ()> {
    async {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
            .await
            .unwrap();
        axum::serve(listener, app()).await.unwrap();
    }
}
