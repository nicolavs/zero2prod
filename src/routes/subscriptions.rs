use crate::ValidatedForm;
use axum::http::StatusCode;
use serde::Deserialize;
use validator::Validate;

use axum::extract::State;
use chrono::Utc;
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct FormData {
    #[validate(length(min = 2, message = "Can not be empty"))]
    email: String,

    #[validate(length(min = 2, message = "Can not be empty"))]
    name: String,
}

pub async fn subscribe(
    State(pool): State<PgPool>,
    ValidatedForm(form): ValidatedForm<FormData>,
) -> StatusCode {
    sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(&pool)
    .await
    .unwrap();

    StatusCode::OK
}
