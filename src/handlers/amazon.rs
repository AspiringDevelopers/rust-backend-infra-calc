use crate::{models::ApiResponse, AppState};
use axum::{
    extract::{Path, State},
    response::Json,
};

#[allow(dead_code)]
pub async fn handle_amazon_webapp(
    State(_state): State<AppState>,
    Path((app, param)): Path<(String, String)>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "app": app,
        "param": param,
        "message": "Amazon webapp handled"
    })))
}
