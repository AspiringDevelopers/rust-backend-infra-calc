use axum::{http::Request, middleware::Next, response::Response};
use tower_http::cors::CorsLayer;

pub fn cors() -> CorsLayer {
    CorsLayer::permissive()
}

pub async fn logger(request: Request<axum::body::Body>, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    tracing::info!("{} {}", method, uri);

    let response = next.run(request).await;

    tracing::info!("{} {} - {}", method, uri, response.status());

    response
}
