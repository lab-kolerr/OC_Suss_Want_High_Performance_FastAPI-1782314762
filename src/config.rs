use std::env;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub cors_origin: String,
    pub rate_limit_window_ms: u64,
    pub rate_limit_max_requests: u64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().unwrap();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_access_secret = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set");
        let jwt_refresh_secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
        let cors_origin = env::var("CORS_ORIGIN").expect("CORS_ORIGIN must be set");
        let rate_limit_window_ms = env::var("RATE_LIMIT_WINDOW_MS").unwrap_or_else(|_| "900000".to_string()).parse().unwrap();
        let rate_limit_max_requests = env::var("RATE_LIMIT_MAX_REQUESTS").unwrap_or_else(|_| "100".to_string()).parse().unwrap();

        Config { port, database_url, jwt_access_secret, jwt_refresh_secret, cors_origin, rate_limit_window_ms, rate_limit_max_requests }
    }
}