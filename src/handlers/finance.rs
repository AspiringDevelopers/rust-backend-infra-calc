use crate::{models::ApiResponse, AppState};
use axum::{extract::State, response::Json, Form};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FinanceForm {
    pub action: String,
}

#[allow(dead_code)]
pub async fn handle_finance_get(
    State(_state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "message": "Finance record keeper"
    })))
}

#[allow(dead_code)]
pub async fn handle_finance_post(
    State(_state): State<AppState>,
    Form(form): Form<FinanceForm>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "action": form.action,
        "message": "Finance action processed"
    })))
}
