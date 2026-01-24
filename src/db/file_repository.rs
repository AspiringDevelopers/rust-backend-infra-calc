use anyhow::Result;
use chrono::Utc;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Collection, Database as MongoDatabase,
};
use uuid::Uuid;

use crate::models::FileData;

pub struct FileRepository<'a> {
    mongo_db: &'a MongoDatabase,
}

impl<'a> FileRepository<'a> {
    pub fn new(mongo_db: &'a MongoDatabase) -> Self {
        Self { mongo_db }
    }

    fn collection(&self) -> Collection<Document> {
        self.mongo_db.collection("files")
    }

    pub async fn create(&self, user_id: Uuid, path: &str, content: &str) -> Result<FileData> {
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

        self.collection().insert_one(doc, None).await?;

        Ok(file)
    }

    pub async fn get(&self, user_id: Uuid, path: &str) -> Result<Option<FileData>> {
        let filter = doc! {
            "user_id": user_id.to_string(),
            "path": path,
        };

        let doc = self.collection().find_one(filter, None).await?;

        Ok(doc.map(|d| self.doc_to_file_data(&d)))
    }

    pub async fn update(&self, user_id: Uuid, path: &str, content: &str) -> Result<()> {
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

        self.collection().update_one(filter, update, None).await?;

        Ok(())
    }

    pub async fn delete(&self, user_id: Uuid, path: &str) -> Result<()> {
        let filter = doc! {
            "user_id": user_id.to_string(),
            "path": path,
        };

        self.collection().delete_one(filter, None).await?;

        Ok(())
    }

    pub async fn list(&self, user_id: Uuid, path_prefix: &str) -> Result<Vec<FileData>> {
        let filter = doc! {
            "user_id": user_id.to_string(),
            "path": {
                "$regex": format!("^{}", regex::escape(path_prefix)),
            }
        };

        let mut cursor = self.collection().find(filter, None).await?;
        let mut files = Vec::new();

        while let Some(result) = cursor.next().await {
            let doc = result?;
            files.push(self.doc_to_file_data(&doc));
        }

        Ok(files)
    }

    fn doc_to_file_data(&self, doc: &Document) -> FileData {
        FileData {
            id: Uuid::parse_str(doc.get_str("_id").unwrap()).unwrap(),
            user_id: Uuid::parse_str(doc.get_str("user_id").unwrap()).unwrap(),
            path: doc.get_str("path").unwrap().to_string(),
            content: doc.get_str("content").unwrap().to_string(),
            created_at: chrono::DateTime::parse_from_rfc3339(doc.get_str("created_at").unwrap())
                .unwrap()
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(doc.get_str("updated_at").unwrap())
                .unwrap()
                .with_timezone(&Utc),
        }
    }
}
