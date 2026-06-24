use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpResponse};
use std::time::Instant;

pub async fn logging_middleware(req: ServiceRequest, next: &dyn Fn(ServiceRequest) -> Result<ServiceResponse, Error>) -> Result<ServiceResponse, Error> {
    let start = Instant::now();
    let res = next(req).await?;
    let duration = start.elapsed();
    println!("Request took: {:?}", duration);
    Ok(res)
}

pub async fn cors_middleware(req: ServiceRequest) -> Result<ServiceResponse, Error> {
    let res = req.error_response(HttpResponse::Forbidden());
    Ok(res)
}

pub async fn request_id_middleware(req: ServiceRequest) -> Result<ServiceResponse, Error> {
    req.extensions_mut().insert::<String>("request_id".to_string());
    Ok(req)
}

pub async fn rate_limiting_middleware(req: ServiceRequest) -> Result<ServiceResponse, Error> {
    // Implement rate limiting logic here
    Ok(req)
}