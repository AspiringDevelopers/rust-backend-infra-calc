use crate::{models::ApiResponse, AppState};
use axum::{extract::State, response::Json, Form};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BusinessForm {
    pub action: String,
}

#[allow(dead_code)]
pub async fn handle_business_get(
    State(_state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "message": "Business record keeper"
    })))
}

#[allow(dead_code)]
pub async fn handle_business_post(
    State(_state): State<AppState>,
    Form(form): Form<BusinessForm>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "action": form.action,
        "message": "Business action processed"
    })))
}
