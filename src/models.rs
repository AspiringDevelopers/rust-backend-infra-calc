use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub dongle: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileData {
    pub id: Uuid,
    pub user_id: Uuid,
    pub path: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InAppPurchase {
    pub id: Uuid,
    pub user_id: Uuid,
    pub app_name: String,
    pub owned: i32,
    pub consumed: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFileRequest {
    pub fname: String,
    pub data: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct WebAppRequest {
    pub action: String,
    pub appname: Option<String>,
    pub fname: Option<String>,
    pub data: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailRequest {
    pub to: String,
    pub subject: String,
    pub text: String,
    pub data: String,
    pub appname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub result: String,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            result: "ok".to_string(),
            data: Some(data),
            message: None,
        }
    }

    #[allow(dead_code)]
    pub fn success_simple() -> Self
    where
        T: Default,
    {
        Self {
            result: "ok".to_string(),
            data: None,
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            result: "fail".to_string(),
            data: None,
            message: Some(message),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileListEntry {
    pub fname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: i64,    // Expiration time
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: Uuid,
    pub email: String,
}
