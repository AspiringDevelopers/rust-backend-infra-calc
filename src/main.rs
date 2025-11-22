use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::fmt::init;

mod auth;
mod config;
mod db;
mod handlers;
mod models;
mod services;
mod utils;

use config::AppConfig;
use db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init();

    let config = AppConfig::from_env()?;
    let db = Database::new(&config.mongo_uri, &config.mongo_database, &config.mysql_dsn).await?;

    let port = config.port;
    let static_path = config.static_path.clone();
    let state = AppState { db, config };

    let api_routes = Router::new()
        .route("/auth/register", post(handlers::auth::api_register))
        .route("/auth/login", post(handlers::auth::api_login))
        .route("/auth/logout", post(handlers::auth::api_logout))
        .route("/auth/me", get(handlers::auth::api_me))
        .route("/files", get(handlers::save::list_files))
        .with_state(state.clone());

    let app = Router::new()
        .route("/", get(handlers::home_page))
        .route("/dashboard", get(handlers::auth::dashboard_page))
        .nest("/api", api_routes)
        .route("/dev", get(handlers::home_page))
        .route(
            "/save",
            get(handlers::save::list_files).post(handlers::save::save_file),
        )
        .route("/runas", get(handlers::run_as::run_app))
        .route("/runasemailer", post(handlers::email::send_email))
        .route("/usersheet", post(handlers::user_sheet::handle_user_sheet))
        .route("/insert", post(handlers::insert::get_file))
        .route(
            "/import",
            get(handlers::import::import_page).post(handlers::import::handle_import),
        )
        .route("/downloadfile", post(handlers::download::download_file))
        .route(
            "/htmltopdf",
            get(handlers::pdf::get_pdf).post(handlers::pdf::convert_html_to_pdf),
        )
        .route(
            "/iconimg",
            get(handlers::image::get_image).post(handlers::image::upload_image),
        )
        .route(
            "/login",
            get(handlers::auth::login_page).post(handlers::auth::login),
        )
        .route(
            "/logout",
            get(handlers::auth::logout).post(handlers::auth::logout),
        )
        .route(
            "/register",
            get(handlers::auth::register_page).post(handlers::auth::register),
        )
        .route(
            "/lostpw",
            get(handlers::auth::lost_password_page).post(handlers::auth::lost_password),
        )
        .route(
            "/webapp",
            get(handlers::webapp::handle_webapp).post(handlers::webapp::handle_webapp_post),
        )
        .route(
            "/pwreset",
            get(handlers::auth::password_reset_page).post(handlers::auth::password_reset),
        )
        .route(
            "/dropbox",
            get(handlers::dropbox::handle_dropbox).post(handlers::dropbox::handle_dropbox_post),
        )
        .route("/inapp", post(handlers::inapp::handle_inapp))
        .route(
            "/restore",
            get(handlers::restore::handle_restore).post(handlers::restore::handle_restore_post),
        )
        .route(
            "/amazonwebapp/:app/randomCode/:param",
            get(handlers::amazon::handle_amazon_webapp),
        )
        .route(
            "/finrecord",
            get(handlers::finance::handle_finance_get).post(handlers::finance::handle_finance_post),
        )
        .route(
            "/bisrecord",
            get(handlers::business::handle_business_get)
                .post(handlers::business::handle_business_post),
        )
        .nest_service("/static", ServeDir::new(&static_path))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server starting on http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}
