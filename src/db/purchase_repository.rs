use anyhow::Result;
use chrono::Utc;
use sqlx::{MySql, Pool, Row};
use uuid::Uuid;

use crate::models::InAppPurchase;

pub struct PurchaseRepository<'a> {
    pool: Option<&'a Pool<MySql>>,
}

impl<'a> PurchaseRepository<'a> {
    pub fn new(pool: Option<&'a Pool<MySql>>) -> Self {
        Self { pool }
    }

    fn get_pool(&self) -> Result<&Pool<MySql>> {
        self.pool
            .ok_or_else(|| anyhow::anyhow!("MySQL not available"))
    }

    pub async fn get_or_create(&self, user_id: Uuid, app_name: &str) -> Result<InAppPurchase> {
        let pool = self.get_pool()?;

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

    pub async fn update(
        &self,
        user_id: Uuid,
        app_name: &str,
        owned: i32,
        consumed: i32,
    ) -> Result<()> {
        let pool = self.get_pool()?;

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
