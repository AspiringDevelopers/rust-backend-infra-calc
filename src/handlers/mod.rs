pub mod amazon;
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
    response::{Html, IntoResponse, Json},
};
use serde_json::json;

pub async fn home(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "message": "Aspiring Investments API",
        "version": "1.0.0",
        "status": "running"
    }))
}

pub async fn home_page(State(_state): State<AppState>) -> impl IntoResponse {
    let html = std::fs::read_to_string("web/templates/home.html")
        .unwrap_or_else(|_| "<h1>Welcome to TouchCalc</h1>".to_string());
    Html(html)
}
