use crate::{models::ApiResponse, AppState};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
    Form,
};
use serde::Deserialize;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct InsertForm {
    pub filename: String,
}

#[allow(dead_code)]
pub async fn get_file(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Form(form): Form<InsertForm>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let file_path = format!("home/{}", form.filename);

    match state.db.get_file(user_id, &file_path).await {
        Ok(Some(file)) => Ok(Json(ApiResponse::success(serde_json::json!({
            "data": file.content,
            "result": "ok"
        })))),
        Ok(None) => Ok(Json(ApiResponse::error("File not found".to_string()))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
