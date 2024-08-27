use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
}

pub async fn users(
    State(pg_pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows = sqlx::query_as!(User, "SELECT name, email FROM USERS")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;
    Ok(Json(rows))
}
