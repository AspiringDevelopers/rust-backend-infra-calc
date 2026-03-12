use crate::{models::ApiResponse, AppState};
use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RunAsQuery {
    pub sheets: Option<String>,
    pub file: String,
}

#[allow(dead_code)]
pub async fn run_app(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Query(query): Query<RunAsQuery>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let file_path = format!("home/{}", query.file);

    match state.db.get_file(user_id, &file_path).await {
        Ok(Some(file)) => {
            let sheets = query
                .sheets
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            Ok(Json(ApiResponse::success(json!({
                "fname": query.file,
                "sheetstr": file.content,
                "sheets": sheets
            }))))
        }
        Ok(None) => Ok(Json(ApiResponse::error("File not found".to_string()))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
