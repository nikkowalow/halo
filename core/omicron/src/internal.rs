use crate::DB_POOL;
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

#[derive(Deserialize)]
pub struct TicketPurchaseRequest {
    pub user_id: i32,
    pub event_id: i32,
    pub quantity: i64,
}

#[derive(Serialize)]
pub struct TicketPurchaseResponse {
    ticket_id: i32,
    event_name: String,
}

pub async fn purchase_ticket(
    Json(request): Json<TicketPurchaseRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pg_pool = DB_POOL.get().expect("DB_POOL must be initialized");
    let available_tickets = sqlx::query_scalar!(
        "SELECT available FROM events WHERE id = $1",
        request.event_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if available_tickets < Some(request.quantity) {
        let error_response = serde_json::json!({
            "status": "error",
            "message": "Not enough tickets available",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let ticket = sqlx::query_as!(
        TicketPurchaseResponse,
        r#"
        WITH new_ticket AS (
            INSERT INTO tickets (user_id, event_id)
            VALUES ($1, $2)
            RETURNING id as ticket_id
        ),
        updated_event AS (
            UPDATE events
            SET available = available - 1
            WHERE id = $2
            RETURNING name as event_name
        )
        SELECT new_ticket.ticket_id, updated_event.event_name
        FROM new_ticket, updated_event
        "#,
        request.user_id,
        request.event_id,
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    Ok((StatusCode::CREATED, Json(ticket)))
}
