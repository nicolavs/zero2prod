use axum::{
    extract::{rejection::FormRejection, Form, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;
use validator::Validate;
#[derive(Debug, Deserialize, Validate)]
struct FormData {
    #[validate(length(min = 2, message = "Can not be empty"))]
    email: String,

    #[validate(length(min = 2, message = "Can not be empty"))]
    name: String,
}

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

use tokio::net::TcpListener;

async fn greet(Path(user_name): Path<String>) -> String {
    format!("Hello {}!", user_name)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

// Let's start simple: we always return a 200 OK
async fn subscribe(ValidatedForm(_form_data): ValidatedForm<FormData>) -> StatusCode {
    StatusCode::OK
}

fn app() -> Router {
    Router::new()
        .route("/{name}", get(greet))
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run(listener: TcpListener) {
    axum::serve(listener, app()).await.unwrap();
}
