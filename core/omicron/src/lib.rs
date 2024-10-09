pub mod internal;
pub mod public;
pub mod types;
pub mod users;

use anyhow::{Context, Result};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use colored::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tokio::net::TcpListener;
use tokio::sync::OnceCell;
use tokio_postgres::Error;
use tower_http::cors::{Any, CorsLayer};

// Define the global PgPool wrapped in OnceCell for shared access
static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn initialize_db_pool() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await
        .expect("connection error...");

    DB_POOL.set(pool).expect("DB_POOL can only be set once");
}

#[tokio::main]
pub async fn run() -> Result<(), Error> {
    println!("{} {} {}", "starting", "OMICRON".purple().bold(), "...");

    initialize_db_pool().await;

    let server_address = env::var("API_ADDRESS").expect("SERVER_ADDRESS not set");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/users", get(internal::users))
        .route("/events", get(public::events))
        .route("/tickets/:event_id", get(public::tickets))
        .with_state(DB_POOL.get().expect("DB_POOL must be initialized").clone())
        .layer(cors);

    let listener = TcpListener::bind(&server_address)
        .await
        .expect("error creating TCP listener...");

    axum::serve(listener, app)
        .await
        .expect("error serving app...");
    Ok(())
}

#[derive(Serialize, Debug)]
pub struct EventPartial {
    pub id: i32,
    pub name: String,
    pub capacity: i64,
    pub available: Option<i64>,
}

pub async fn event(event_id: i32) -> Result<EventPartial> {
    let pool = DB_POOL.get().expect("DB_POOL must be initialized");

    let row = sqlx::query!(
        r#"
        SELECT 
            id,
            name,
            capacity,
            available
        FROM events
        WHERE id = $1
        "#,
        event_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to fetch event from database")?;

    let event_partial = EventPartial {
        id: row.id,
        name: row.name,
        capacity: row.capacity,
        available: row.available,
    };

    Ok(event_partial)
}
