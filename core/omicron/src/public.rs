use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "event_category")]
#[derive(Serialize, Deserialize)]
pub enum EventCategory {
    Festival,
    Concert,
    Club,
    Birthday,
    Dinner,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    id: i32,
    name: String,
    location: String,
    address: String,
    category: EventCategory,
    capacity: i64,
    available: Option<i64>,
    created_at: chrono::NaiveDate,
    updated_at: Option<chrono::NaiveDate>,
}

#[derive(Serialize, Deserialize)]
pub struct Ticket {
    id: i32,
    event_id: i32,
    price: f64,
    ticket_type: Option<TicketType>,
    seat: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "ticket_type")]
#[derive(Serialize, Deserialize)]
pub enum TicketType {
    GA,
    VIP,
}

pub async fn events(
    State(pg_pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows = sqlx::query_as!(
        Event,
        r#"
        SELECT 
            id, 
            name,
            location,
            address,
            category as "category: EventCategory",
            capacity,
            available,
            created_at,
            updated_at
        FROM events
        "#
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    Ok(Json(rows))
}

pub async fn tickets(
    Path(event_id): Path<i32>,
    State(pg_pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows = sqlx::query_as!(
        Ticket,
        r#"
        SELECT 
            id,
            event_id,
            price,
            ticket_type as "ticket_type: TicketType",
            seat
        FROM tickets
        WHERE event_id = $1
        "#,
        event_id
    )
    .fetch_all(&pg_pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    Ok(Json(rows))
}
