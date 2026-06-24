use axum::{http::StatusCode, Router};
use hyper::Body;
use tokio;

#[tokio::test]
async fn test_login() {
    let app = Router::new().route("/login", axum::routing::post(login));
    let response = app.oneshot(axum::http::Request::post("/login").body(Body::from("{\"email\":\"test@example.com\",\"password\":\"password\"}")).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_invalid_login() {
    let app = Router::new().route("/login", axum::routing::post(login));
    let response = app.oneshot(axum::http::Request::post("/login").body(Body::from("{\"email\":\"wrong@example.com\",\"password\":\"wrong_password\"}")).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}