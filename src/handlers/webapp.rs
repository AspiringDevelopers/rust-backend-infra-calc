use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::Json,
    Form,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{models::ApiResponse, AppState};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WebAppQuery {
    pub action: String,
    pub appname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WebAppForm {
    pub action: String,
    pub appname: Option<String>,
    pub fname: Option<String>,
    pub data: Option<String>,
    #[allow(dead_code)]
    pub uuid: Option<String>,
    #[allow(dead_code)]
    pub password: Option<String>,
    #[serde(rename = "deviceId")]
    #[allow(dead_code)]
    pub device_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct InAppStatus {
    pub owned: i32,
    pub consumed: i32,
}

#[allow(dead_code)]
pub async fn handle_webapp(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Query(query): Query<WebAppQuery>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match query.action.as_str() {
        "login" => Ok(Json(ApiResponse::success(json!({"status": "ok"})))),
        "getInapp" => {
            let app_name = query.appname.unwrap_or_default();
            let purchase = match state.db.get_or_create_purchase(user_id, &app_name).await {
                Ok(purchase) => purchase,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };

            let status = InAppStatus {
                owned: purchase.owned,
                consumed: purchase.consumed,
            };

            Ok(Json(ApiResponse::success(json!(status))))
        }
        _ => Ok(Json(ApiResponse::error("Unknown action".to_string()))),
    }
}

pub async fn handle_webapp_post(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Form(form): Form<WebAppForm>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    match form.action.as_str() {
        "savefile" => {
            let app_name = form.appname.unwrap_or_default();
            let fname = form.fname.unwrap_or_default();
            let data = form.data.unwrap_or_default();

            // Check in-app purchase limits
            let purchase = match state.db.get_or_create_purchase(user_id, &app_name).await {
                Ok(purchase) => purchase,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };

            if purchase.owned - purchase.consumed <= 0 {
                return Ok(Json(ApiResponse::success(json!({"result": "buy"}))));
            }

            let file_path = format!("home/securestore/{}/{}", app_name, fname);

            // Save file
            match state.db.get_file(user_id, &file_path).await {
                Ok(Some(_)) => {
                    if state
                        .db
                        .update_file(user_id, &file_path, &data)
                        .await
                        .is_err()
                    {
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
                Ok(None) => {
                    if state
                        .db
                        .create_file(user_id, &file_path, &data)
                        .await
                        .is_err()
                    {
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }

            Ok(Json(ApiResponse::success(json!({"result": "ok"}))))
        }
        "savecurrentfile" => {
            let app_name = form.appname.unwrap_or_default();
            let fname = form.fname.unwrap_or_default();
            let data = form.data.unwrap_or_default();
            let file_path = format!("home/securestore/{}/{}", app_name, fname);

            // Check if file exists before updating
            match state.db.get_file(user_id, &file_path).await {
                Ok(Some(_)) => {
                    if state
                        .db
                        .update_file(user_id, &file_path, &data)
                        .await
                        .is_err()
                    {
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                    Ok(Json(ApiResponse::success(json!({"result": "ok"}))))
                }
                Ok(None) => Ok(Json(ApiResponse::error("File not found".to_string()))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        "getfile" => {
            let app_name = form.appname.unwrap_or_default();
            let fname = form.fname.unwrap_or_default();
            let file_path = format!("home/securestore/{}/{}", app_name, fname);

            match state.db.get_file(user_id, &file_path).await {
                Ok(Some(file)) => Ok(Json(ApiResponse::success(json!({
                    "data": file.content,
                    "result": "ok"
                })))),
                Ok(None) => Ok(Json(ApiResponse::error("File not found".to_string()))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        "deletefile" => {
            let app_name = form.appname.unwrap_or_default();
            let fname = form.fname.unwrap_or_default();
            let file_path = format!("home/securestore/{}/{}", app_name, fname);

            if state.db.delete_file(user_id, &file_path).await.is_err() {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }

            Ok(Json(ApiResponse::success(json!({"result": "ok"}))))
        }
        "listdir" => {
            let app_name = form.appname.unwrap_or_default();
            let dir_path = format!("home/securestore/{}/", app_name);

            let files = match state.db.list_files(user_id, &dir_path).await {
                Ok(files) => files,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };

            let entries: Vec<String> = files
                .into_iter()
                .map(|file| file.path.split('/').next_back().unwrap_or("").to_string())
                .collect();

            Ok(Json(ApiResponse::success(json!({
                "data": entries,
                "result": "ok"
            }))))
        }
        "update" => {
            let app_name = form.appname.unwrap_or_default();
            let mut purchase = match state.db.get_or_create_purchase(user_id, &app_name).await {
                Ok(purchase) => purchase,
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };

            purchase.consumed += 1;

            // If all saves are consumed, reset
            if purchase.consumed >= purchase.owned {
                purchase.owned = 0;
                purchase.consumed = 0;
            }

            if state
                .db
                .update_purchase(user_id, &app_name, purchase.owned, purchase.consumed)
                .await
                .is_err()
            {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }

            Ok(Json(ApiResponse::success(json!({"result": "ok"}))))
        }
        "purchase" => {
            let app_name = form.appname.unwrap_or_default();

            if state
                .db
                .update_purchase(user_id, &app_name, 10, 0)
                .await
                .is_err()
            {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }

            Ok(Json(ApiResponse::success(json!({"result": "ok"}))))
        }
        _ => Ok(Json(ApiResponse::error("Unknown action".to_string()))),
    }
}
