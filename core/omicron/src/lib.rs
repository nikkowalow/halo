pub mod internal;
pub mod public;
pub mod types;
pub mod users;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use colored::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tokio::net::TcpListener;
use tokio_postgres::Error;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
pub async fn run() -> Result<(), Error> {
    println!("{} {} {}", "starting", "omicron".green().bold(), "...");
    dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await
        .expect("connection error...");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/users", get(internal::users))
        .route("/events", get(public::events))
        .route("/tickets/:event_id", get(public::tickets))
        .with_state(db_pool)
        .layer(cors);

    let listener = TcpListener::bind(server_address)
        .await
        .expect("error creating TCP listener...");

    axum::serve(listener, app)
        .await
        .expect("error serving app...");
    Ok(())
}
