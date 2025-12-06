use crate::models::{FileData, InAppPurchase, User};
use anyhow::Result;
use chrono::Utc;
use mongodb::{
    bson::{doc, Document},
    Client as MongoClient, Collection, Database as MongoDatabase,
};
use sqlx::{MySql, Pool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    mongo_client: MongoClient,
    mongo_db: MongoDatabase,
    mysql_pool: Option<Pool<MySql>>,
}

impl Database {
    pub async fn new(mongo_uri: &str, mongo_db_name: &str, mysql_dsn: &str) -> Result<Self> {
        // Connect to MongoDB
        let mongo_client = MongoClient::with_uri_str(mongo_uri).await?;
        let mongo_db = mongo_client.database(mongo_db_name);

        // Try to connect to MySQL - make it optional
        let mysql_pool = match Self::connect_mysql(mysql_dsn).await {
            Ok(pool) => Some(pool),
            Err(e) => {
                eprintln!(
                    "Warning: MySQL connection failed: {}. Continuing without MySQL.",
                    e
                );
                None
            }
        };

        Ok(Self {
            mongo_client,
            mongo_db,
            mysql_pool,
        })
    }

    async fn connect_mysql(mysql_dsn: &str) -> Result<Pool<MySql>> {
        let mysql_pool = Pool::<MySql>::connect(mysql_dsn).await?;
        Self::init_mysql_tables(&mysql_pool).await?;
        Ok(mysql_pool)
    }

    async fn init_mysql_tables(pool: &Pool<MySql>) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id CHAR(36) PRIMARY KEY,
                email VARCHAR(255) UNIQUE NOT NULL,
                password_hash VARCHAR(255) NOT NULL,
                dongle VARCHAR(255),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS in_app_purchases (
                id CHAR(36) PRIMARY KEY,
                user_id CHAR(36) NOT NULL,
                app_name VARCHAR(255) NOT NULL,
                owned INT NOT NULL DEFAULT 0,
                consumed INT NOT NULL DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub fn mongo_db(&self) -> &MongoDatabase {
        &self.mongo_db
    }

    pub fn mysql_pool(&self) -> Option<&Pool<MySql>> {
        self.mysql_pool.as_ref()
    }

    fn files_collection(&self) -> Collection<Document> {
        self.mongo_db.collection("files")
    }

    // User operations (MySQL)
    pub async fn create_user(&self, email: &str, password_hash: &str) -> Result<User> {
        let pool = self
            .mysql_pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("MySQL not available"))?;

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(email)
        .bind(password_hash)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        Ok(User {
            id,
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            dongle: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let pool = self
            .mysql_pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("MySQL not available"))?;

        let row = sqlx::query(
            "SELECT id, email, password_hash, dongle, created_at, updated_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| User {
            id: Uuid::parse_str(r.get("id")).unwrap(),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            dongle: r.get("dongle"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    pub async fn update_user_password(&self, user_id: Uuid, password_hash: &str) -> Result<()> {
        let pool = self
            .mysql_pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("MySQL not available"))?;

        sqlx::query("UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?")
            .bind(password_hash)
            .bind(Utc::now())
            .bind(user_id.to_string())
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn set_user_dongle(&self, user_id: Uuid, dongle: &str) -> Result<()> {
        let pool = self
            .mysql_pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("MySQL not available"))?;

        sqlx::query("UPDATE users SET dongle = ?, updated_at = ? WHERE id = ?")
            .bind(dongle)
            .bind(Utc::now())
            .bind(user_id.to_string())
            .execute(pool)
            .await?;

        Ok(())
    }

    // File operations (MongoDB)
    pub async fn create_file(&self, user_id: Uuid, path: &str, content: &str) -> Result<FileData> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let file = FileData {
            id,
            user_id,
            path: path.to_string(),
            content: content.to_string(),
            created_at: now,
            updated_at: now,
        };

        let doc = doc! {
            "_id": id.to_string(),
            "user_id": user_id.to_string(),
            "path": path,
            "content": content,
            "created_at": now.to_rfc3339(),
            "updated_at": now.to_rfc3339(),
        };

        self.files_collection().insert_one(doc, None).await?;

        Ok(file)
    }

    pub async fn get_file(&self, user_id: Uuid, path: &str) -> Result<Option<FileData>> {
        let filter = doc! {
            "user_id": user_id.to_string(),
            "path": path,
        };

        let doc = self.files_collection().find_one(filter, None).await?;

        Ok(doc.map(|d| FileData {
            id: Uuid::parse_str(d.get_str("_id").unwrap()).unwrap(),
            user_id: Uuid::parse_str(d.get_str("user_id").unwrap()).unwrap(),
            path: d.get_str("path").unwrap().to_string(),
            content: d.get_str("content").unwrap().to_string(),
            created_at: chrono::DateTime::parse_from_rfc3339(d.get_str("created_at").unwrap())
                .unwrap()
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(d.get_str("updated_at").unwrap())
                .unwrap()
                .with_timezone(&Utc),
        }))
    }

    pub async fn update_file(&self, user_id: Uuid, path: &str, content: &str) -> Result<()> {
        let filter = doc! {
            "user_id": user_id.to_string(),
            "path": path,
        };

        let update = doc! {
            "$set": {
                "content": content,
                "updated_at": Utc::now().to_rfc3339(),
            }
        };

        self.files_collection()
            .update_one(filter, update, None)
            .await?;

        Ok(())
    }

    pub async fn delete_file(&self, user_id: Uuid, path: &str) -> Result<()> {
        let filter = doc! {
            "user_id": user_id.to_string(),
            "path": path,
        };

        self.files_collection().delete_one(filter, None).await?;

        Ok(())
    }

    pub async fn list_files(&self, user_id: Uuid, path_prefix: &str) -> Result<Vec<FileData>> {
        let filter = doc! {
            "user_id": user_id.to_string(),
            "path": {
                "$regex": format!("^{}", regex::escape(path_prefix)),
            }
        };

        let mut cursor = self.files_collection().find(filter, None).await?;
        let mut files = Vec::new();

        use futures::stream::StreamExt;
        while let Some(result) = cursor.next().await {
            let doc = result?;
            files.push(FileData {
                id: Uuid::parse_str(doc.get_str("_id").unwrap()).unwrap(),
                user_id: Uuid::parse_str(doc.get_str("user_id").unwrap()).unwrap(),
                path: doc.get_str("path").unwrap().to_string(),
                content: doc.get_str("content").unwrap().to_string(),
                created_at: chrono::DateTime::parse_from_rfc3339(
                    doc.get_str("created_at").unwrap(),
                )
                .unwrap()
                .with_timezone(&Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(
                    doc.get_str("updated_at").unwrap(),
                )
                .unwrap()
                .with_timezone(&Utc),
            });
        }

        Ok(files)
    }

    // In-app purchase operations (MySQL)
    pub async fn get_or_create_purchase(
        &self,
        user_id: Uuid,
        app_name: &str,
    ) -> Result<InAppPurchase> {
        let pool = self
            .mysql_pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("MySQL not available"))?;

        // Try to get existing purchase
        let row = sqlx::query(
            "SELECT id, user_id, app_name, owned, consumed, created_at, updated_at FROM in_app_purchases WHERE user_id = ? AND app_name = ?"
        )
        .bind(user_id.to_string())
        .bind(app_name)
        .fetch_optional(pool)
        .await?;

        if let Some(r) = row {
            return Ok(InAppPurchase {
                id: Uuid::parse_str(r.get("id")).unwrap(),
                user_id: Uuid::parse_str(r.get("user_id")).unwrap(),
                app_name: r.get("app_name"),
                owned: r.get("owned"),
                consumed: r.get("consumed"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            });
        }

        // Create new purchase
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO in_app_purchases (id, user_id, app_name, owned, consumed, created_at, updated_at) VALUES (?, ?, ?, 0, 0, ?, ?)"
        )
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(app_name)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;

        Ok(InAppPurchase {
            id,
            user_id,
            app_name: app_name.to_string(),
            owned: 0,
            consumed: 0,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn update_purchase(
        &self,
        user_id: Uuid,
        app_name: &str,
        owned: i32,
        consumed: i32,
    ) -> Result<()> {
        let pool = self
            .mysql_pool
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("MySQL not available"))?;

        sqlx::query(
            "UPDATE in_app_purchases SET owned = ?, consumed = ?, updated_at = ? WHERE user_id = ? AND app_name = ?"
        )
        .bind(owned)
        .bind(consumed)
        .bind(Utc::now())
        .bind(user_id.to_string())
        .bind(app_name)
        .execute(pool)
        .await?;

        Ok(())
    }
}
