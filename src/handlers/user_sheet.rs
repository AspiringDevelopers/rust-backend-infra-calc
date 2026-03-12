use crate::{models::ApiResponse, AppState};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
    Form,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserSheetForm {
    pub pagename: String,
    pub delete: Option<String>,
}

pub async fn handle_user_sheet(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Form(form): Form<UserSheetForm>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    if form.delete == Some("yes".to_string()) {
        let file_path = format!("home/{}", form.pagename);
        if state.db.delete_file(user_id, &file_path).await.is_err() {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        return Ok(Json(ApiResponse::success(
            serde_json::json!({"deleted": true}),
        )));
    }

    Ok(Json(ApiResponse::success(
        serde_json::json!({"message": "user sheet handled"}),
    )))
}
