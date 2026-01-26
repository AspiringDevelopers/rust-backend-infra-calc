use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::CookieJar;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

use crate::handlers::get_current_user_id;

#[allow(dead_code)]
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

pub async fn auth_guard(jar: CookieJar, mut request: Request, next: Next) -> Response {
    let user_id = match get_current_user_id(&jar) {
        Some(id) => id,
        None => return Redirect::to("/login").into_response(),
    };
    request.extensions_mut().insert(user_id);
    next.run(request).await
}
