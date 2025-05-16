//! src/lib.rs
pub mod configuration;
pub mod routes;
pub mod startup;

use axum::{
    extract::{rejection::FormRejection, Form, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

use axum::extract::Path;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

async fn greet(Path(user_name): Path<String>) -> String {
    format!("Hello {}!", user_name)
}

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

pub async fn new_pgpool(db_connection_str: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(db_connection_str)
        .await
        .expect("can't connect to database")
}
