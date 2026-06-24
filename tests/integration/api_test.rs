use axum::{http::StatusCode, Router};
use hyper::Body;
use tokio;

#[tokio::test]
async fn test_get_users() {
    let app = Router::new().route("/users", axum::routing::get(get_users));
    let response = app.oneshot(axum::http::Request::get("/users").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user() {
    let app = Router::new().route("/users", axum::routing::post(create_user));
    let response = app.oneshot(axum::http::Request::post("/users").body(Body::from("{\"email\":\"test@example.com\",\"password\":\"password\"}")).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}