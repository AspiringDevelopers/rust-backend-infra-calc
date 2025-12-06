use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::CookieJar;
use tower_http::cors::CorsLayer;

pub fn cors() -> CorsLayer {
    CorsLayer::permissive()
}

pub async fn logger(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    tracing::info!("{} {}", method, uri);
    let response = next.run(request).await;
    tracing::info!("{} {} - {}", method, uri, response.status());
    response
}

pub async fn auth_guard(jar: CookieJar, request: Request, next: Next) -> Response {
    if let Some(cookie) = jar.get("user") {
        if !cookie.value().is_empty() {
            return next.run(request).await;
        }
    }
    Redirect::to("/login").into_response()
}
