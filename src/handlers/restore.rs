use crate::{models::ApiResponse, AppState};
use axum::{extract::State, response::Json, Form};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RestoreForm {
    pub action: String,
    pub appname: String,
}

#[allow(dead_code)]
pub async fn handle_restore(
    State(_state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "message": "Restore functionality"
    })))
}

#[allow(dead_code)]
pub async fn handle_restore_post(
    State(_state): State<AppState>,
    Form(form): Form<RestoreForm>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "action": form.action,
        "appname": form.appname,
        "message": "Restore processed"
    })))
}
