use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub environment: String,
    pub port: u16,
    pub jwt_secret: String,
    pub cookie_secret: String,

    // Storage
    pub storage_backend: String,

    // MongoDB
    pub mongo_uri: String,
    pub mongo_database: String,

    // MySQL
    pub mysql_dsn: String,

    // MinIO/S3
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub minio_bucket: String,
    pub minio_ssl: bool,

    // AWS (fallback)
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub s3_bucket: String,

    // Email
    pub from_email: String,

    // Paths
    pub templates_path: String,
    pub static_path: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "11oETzKXQAGaYdkL5gEmGeJJFuYh7EQnp2XdTP1o/Vo=".to_string()),
            cookie_secret: env::var("COOKIE_SECRET")
                .unwrap_or_else(|_| "11oETzKXQAGaYdkL5gEmGeJJFuYh7EQnp2XdTP1o/Vo=".to_string()),

            storage_backend: env::var("STORAGE_BACKEND").unwrap_or_else(|_| "minio".to_string()),

            mongo_uri: env::var("MONGO_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            mongo_database: env::var("MONGO_DATABASE").unwrap_or_else(|_| "touchcalc".to_string()),

            mysql_dsn: env::var("MYSQL_DSN")
                .unwrap_or_else(|_| "root:password@localhost:3306/touchcalc".to_string()),

            minio_endpoint: env::var("MINIO_ENDPOINT")
                .unwrap_or_else(|_| "localhost:9000".to_string()),
            minio_access_key: env::var("MINIO_ACCESS_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            minio_secret_key: env::var("MINIO_SECRET_KEY")
                .unwrap_or_else(|_| "minioadmin".to_string()),
            minio_bucket: env::var("MINIO_BUCKET")
                .unwrap_or_else(|_| "touchcalc-storage".to_string()),
            minio_ssl: env::var("MINIO_SSL")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),

            aws_access_key_id: env::var("AWS_ACCESS_KEY_ID").unwrap_or_default(),
            aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default(),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            s3_bucket: env::var("S3_BUCKET")
                .unwrap_or_else(|_| "aspiring-cloud-storage".to_string()),

            from_email: env::var("FROM_EMAIL")
                .unwrap_or_else(|_| "aspiring.investments@gmail.com".to_string()),

            templates_path: env::var("TEMPLATES_PATH")
                .unwrap_or_else(|_| "./web/templates".to_string()),
            static_path: env::var("STATIC_PATH").unwrap_or_else(|_| "./web/static".to_string()),
        })
    }
}
