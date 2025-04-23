use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
#[derive(Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32, // user id
    exp: usize,
    email: String,
    name: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
}

pub async fn signup(
    State(pg_pool): State<PgPool>,
    Json(req): Json<SignupRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let password_hash = hash_password(&req.password)?;

    sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        req.name,
        req.email,
        password_hash
    )
    .execute(&pg_pool)
    .await
    .map_err(|e| internal_error(format!("Failed to create user: {}", e)))?;

    Ok(Json(AuthResponse {
        message: "User created successfully".to_string(),
    }))
}

fn hash_password(password: &str) -> Result<String, (StatusCode, Json<serde_json::Value>)> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| internal_error("Failed to hash password".into()))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserData {
    id: i32,
    name: String,
    email: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user: UserData,
}

pub async fn login(
    State(pg_pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let row = sqlx::query!(
        "SELECT id, name, email, password FROM users WHERE email = $1",
        req.email
    )
    .fetch_optional(&pg_pool)
    .await
    .map_err(|e| internal_error(format!("Database error: {}", e)))?;

    let Some(user) = row else {
        return Err(unauthorized("Invalid email or password"));
    };

    let parsed_hash = PasswordHash::new(user.password.as_deref().unwrap_or(""))
        .map_err(|_| internal_error("Stored password hash is invalid".to_string()))?;

    let is_valid = Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return Err(unauthorized("Invalid email or password"));
    }

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        name: user.name.clone(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| internal_error(format!("Failed to generate token: {}", e)))?;

    Ok(Json(LoginResponse {
        token,
        user: UserData {
            id: user.id,
            name: user.name,
            email: user.email,
        },
    }))
}

fn unauthorized(msg: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({ "error": msg })),
    )
}

fn internal_error(msg: String) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({ "error": msg })),
    )
}
