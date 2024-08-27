use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use colored::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tokio::net::TcpListener;
use tokio_postgres::Error;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    println!("{} {} {}", "starting", "omicron".green().bold(), "...");
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await
        .expect("connection error...");

    let app = Router::new()
        .route("/heartbeat", get(heartbeat))
        .route("/users", get(users))
        .with_state(db_pool);

    let listener = TcpListener::bind(server_address)
        .await
        .expect("error creating TCP listener...");

    axum::serve(listener, app)
        .await
        .expect("error serving app...");
    Ok(())
}

pub async fn heartbeat() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

async fn users(
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
