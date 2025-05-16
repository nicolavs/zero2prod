use crate::{greet, routes};
use axum::routing::{get, post};
use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;

fn app(pool: PgPool) -> Router {
    Router::new()
        .route("/{name}", get(greet))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
        .with_state(pool)
}
pub async fn run(listener: TcpListener, db: PgPool) {
    axum::serve(listener, app(db)).await.unwrap();
}
