use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, decode, Header, Validation};
use thiserror::Error;
use std::time::{SystemTime, Duration};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User already exists")] 
    UserExists,
    #[error("Invalid credentials")] 
    InvalidCredentials,
    #[error("Database error")] 
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AuthError::UserExists => axum::http::StatusCode::CONFLICT,
            AuthError::InvalidCredentials => axum::http::StatusCode::UNAUTHORIZED,
            AuthError::DatabaseError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}

pub async fn register(Json(payload): Json<RegisterRequest>, pool: Extension<PgPool>) -> Result<Json<AuthResponse>, AuthError> {
    let hashed_password = hash(&payload.password, 12).map_err(|_| AuthError::DatabaseError(sqlx::Error::RowNotFound))?;
    // Save user to database (pseudo code)
    // sqlx::query("INSERT INTO users (email, password, username) VALUES ($1, $2, $3)")
    //     .bind(&payload.email)
    //     .bind(&hashed_password)
    //     .bind(&payload.username)
    //     .execute(&**pool).await?;

    let access_token = generate_jwt(&payload.email, "user")?;
    let refresh_token = generate_refresh_token(&payload.email)?;

    Ok(Json(AuthResponse { access_token, refresh_token }))
}

pub async fn login(Json(payload): Json<LoginRequest>, pool: Extension<PgPool>) -> Result<Json<AuthResponse>, AuthError> {
    // Fetch user from database (pseudo code)
    // let user = sqlx::query!("SELECT * FROM users WHERE email = $1", &payload.email)
    //     .fetch_one(&**pool).await.map_err(|_| AuthError::InvalidCredentials)?;

    // Verify password
    // if !verify(&payload.password, &user.password).map_err(|_| AuthError::InvalidCredentials)? {
    //     return Err(AuthError::InvalidCredentials);
    // }

    let access_token = generate_jwt(&payload.email, "user")?;
    let refresh_token = generate_refresh_token(&payload.email)?;

    Ok(Json(AuthResponse { access_token, refresh_token }))
}

fn generate_jwt(email: &str, role: &str) -> Result<String, AuthError> {
    let claims = Claims { email: email.to_string(), role: role.to_string(), exp: (SystemTime::now() + Duration::from_secs(900)).duration_since(SystemTime::UNIX_EPOCH)?.as_secs() };
    let token = encode(&Header::default(), &claims, "your_jwt_secret".as_ref()).map_err(|_| AuthError::InvalidCredentials)?;
    Ok(token)
}

fn generate_refresh_token(email: &str) -> Result<String, AuthError> {
    let claims = RefreshClaims { email: email.to_string(), exp: (SystemTime::now() + Duration::from_secs(604800)).duration_since(SystemTime::UNIX_EPOCH)?.as_secs() };
    let token = encode(&Header::default(), &claims, "your_refresh_secret".as_ref()).map_err(|_| AuthError::InvalidCredentials)?;
    Ok(token)
}

#[derive(Debug, Serialize)]
pub struct Claims {
    pub email: String,
    pub role: String,
    pub exp: u64,
}

#[derive(Debug, Serialize)]
pub struct RefreshClaims {
    pub email: String,
    pub exp: u64,
}