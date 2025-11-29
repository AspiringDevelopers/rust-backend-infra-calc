pub mod amazon;
pub mod app;
pub mod auth;
pub mod business;
pub mod download;
pub mod dropbox;
pub mod email;
pub mod finance;
pub mod image;
pub mod import;
pub mod inapp;
pub mod insert;
pub mod pdf;
pub mod restore;
pub mod run_as;
pub mod save;
pub mod user_sheet;
pub mod webapp;

use crate::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json, Redirect},
};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;

// Health check endpoint
pub async fn health_check(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "rust-backend-infra-calc",
        "storage": state.config.storage_backend,
        "templates_loaded": true,
    }))
}

// Home route - redirect based on auth
pub async fn home(jar: CookieJar) -> Result<Redirect, Redirect> {
    if let Some(cookie) = jar.get("user") {
        if !cookie.value().is_empty() {
            return Ok(Redirect::to("/save"));
        }
    }
    Ok(Redirect::to("/login"))
}

pub async fn home_page(State(_state): State<AppState>) -> impl IntoResponse {
    let html = std::fs::read_to_string("web/templates/home.html")
        .unwrap_or_else(|_| "<h1>Welcome to TouchCalc</h1>".to_string());
    Html(html)
}

// Helper function to get current user from cookie jar
pub fn get_current_user(jar: &CookieJar) -> Option<String> {
    jar.get("user").and_then(|cookie| {
        let value = cookie.value();
        // Handle both JSON format and plain text format
        if value.starts_with('"') && value.ends_with('"') {
            // JSON format
            serde_json::from_str::<String>(value).ok()
        } else {
            // Plain text format
            Some(value.to_string())
        }
    })
}

// Helper to set current user cookie
pub fn create_user_cookie(email: &str) -> axum_extra::extract::cookie::Cookie<'static> {
    let user_json = serde_json::to_string(email).unwrap_or_else(|_| format!("\"{}\"", email));
    axum_extra::extract::cookie::Cookie::build(("user", user_json))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::days(30))
        .build()
}

// Helper to clear user cookie
pub fn clear_user_cookie() -> axum_extra::extract::cookie::Cookie<'static> {
    axum_extra::extract::cookie::Cookie::build(("user", ""))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(0))
        .build()
}
