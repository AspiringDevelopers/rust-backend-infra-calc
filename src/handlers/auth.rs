use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json, Redirect},
    Form,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    auth::{create_jwt, hash_password, verify_jwt, verify_password},
    models::{ApiResponse, LoginRequest, RegisterRequest},
    AppState,
};

// HTML Page handlers
pub async fn login_page() -> impl IntoResponse {
    let html = std::fs::read_to_string("web/templates/login.html")
        .unwrap_or_else(|_| "<h1>Login Page</h1>".to_string());
    Html(html)
}

pub async fn register_page() -> impl IntoResponse {
    let html = std::fs::read_to_string("web/templates/register.html")
        .unwrap_or_else(|_| "<h1>Register Page</h1>".to_string());
    Html(html)
}

pub async fn dashboard_page(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<impl IntoResponse, Redirect> {
    // Check if user is authenticated via cookie
    if let Some(cookie) = jar.get("auth_token") {
        if let Ok(claims) = verify_jwt(cookie.value(), &state.config.jwt_secret) {
            // Load template with user info
            let html = std::fs::read_to_string("web/templates/dashboard.html")
                .unwrap_or_else(|_| "<h1>Dashboard</h1>".to_string());

            // Simple template replacement for email
            let html = html.replace("{{ email }}", &claims.sub);

            return Ok(Html(html));
        }
    }

    Err(Redirect::to("/login"))
}

pub async fn logout_page(jar: CookieJar) -> (CookieJar, Redirect) {
    let jar = jar.remove(Cookie::from("auth_token"));
    (jar, Redirect::to("/"))
}

// API handlers
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
}

pub async fn api_register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Validate email
    if !payload.email.contains('@') {
        return Ok(Json(json!({
            "error": "Invalid email address"
        })));
    }

    // Validate password
    if payload.password.len() < 6 {
        return Ok(Json(json!({
            "error": "Password must be at least 6 characters"
        })));
    }

    // Check if user already exists
    match state.db.get_user_by_email(&payload.email).await {
        Ok(Some(_)) => {
            return Ok(Json(json!({
                "error": "User already exists"
            })));
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Hash password
    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Password hashing error: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Create user
    match state.db.create_user(&payload.email, &password_hash).await {
        Ok(user) => Ok(Json(json!({
            "message": "User created successfully",
            "user": {
                "id": user.id.to_string(),
                "email": user.email
            }
        }))),
        Err(e) => {
            tracing::error!("User creation error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn api_login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<(CookieJar, Json<serde_json::Value>), StatusCode> {
    // Get user
    let user = match state.db.get_user_by_email(&payload.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok((
                jar,
                Json(json!({
                    "error": "Invalid credentials"
                })),
            ));
        }
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Verify password
    if !verify_password(&payload.password, &user.password_hash) {
        return Ok((
            jar,
            Json(json!({
                "error": "Invalid credentials"
            })),
        ));
    }

    // Create JWT
    let token = match create_jwt(user.id, &state.config.jwt_secret) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("JWT creation error: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Set cookie
    let cookie = Cookie::build(("auth_token", token.clone()))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::days(7))
        .build();

    let jar = jar.add(cookie);

    Ok((
        jar,
        Json(json!({
            "token": token,
            "user": {
                "id": user.id.to_string(),
                "email": user.email
            }
        })),
    ))
}

pub async fn api_logout(jar: CookieJar) -> (CookieJar, Json<serde_json::Value>) {
    let jar = jar.remove(Cookie::from("auth_token"));
    (
        jar,
        Json(json!({
            "message": "Logged out successfully"
        })),
    )
}

pub async fn api_me(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Check if user is authenticated via cookie
    if let Some(cookie) = jar.get("auth_token") {
        if let Ok(claims) = verify_jwt(cookie.value(), &state.config.jwt_secret) {
            return Ok(Json(json!({
                "email": claims.sub,
                "user_id": claims.user_id
            })));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Keep old handlers for compatibility
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LostPasswordForm {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordResetQuery {
    pub u: String,
    pub d: String,
}

pub async fn login(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let user = match state.db.get_user_by_email(&form.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Ok(Json(ApiResponse::error("Invalid credentials".to_string()))),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if !verify_password(&form.password, &user.password_hash) {
        return Ok(Json(ApiResponse::error("Invalid credentials".to_string())));
    }

    let token = match create_jwt(user.id, &state.config.jwt_secret) {
        Ok(token) => token,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(ApiResponse::success(json!({
        "token": token,
        "user": {
            "id": user.id,
            "email": user.email
        }
    }))))
}

pub async fn logout(State(_state): State<AppState>) -> Json<ApiResponse<()>> {
    Json(ApiResponse::success_simple())
}

pub async fn register(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // Check if user already exists
    match state.db.get_user_by_email(&form.email).await {
        Ok(Some(_)) => return Ok(Json(ApiResponse::error("User already exists".to_string()))),
        Ok(None) => {}
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    let password_hash = match hash_password(&form.password) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match state.db.create_user(&form.email, &password_hash).await {
        Ok(user) => Ok(Json(ApiResponse::success(json!({
            "message": "User registered successfully",
            "user": {
                "id": user.id,
                "email": user.email
            }
        })))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn lost_password_page(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "page": "lost_password",
        "message": "Enter your email to reset password"
    }))
}

pub async fn lost_password(
    State(_state): State<AppState>,
    Form(_form): Form<LostPasswordForm>,
) -> Json<ApiResponse<()>> {
    // Placeholder - implement email sending
    Json(ApiResponse::success_simple())
}

pub async fn password_reset_page(
    State(_state): State<AppState>,
    Query(_query): Query<PasswordResetQuery>,
) -> Json<serde_json::Value> {
    Json(json!({
        "page": "password_reset",
        "message": "Enter your new password"
    }))
}

pub async fn password_reset(
    State(_state): State<AppState>,
    Form(_form): Form<PasswordResetForm>,
) -> Json<ApiResponse<()>> {
    // Placeholder - implement password reset
    Json(ApiResponse::success_simple())
}
