use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub data: HashMap<String, serde_json::Value>,
    pub last_used: DateTime<Utc>,
}

impl Session {
    pub fn new(id: String) -> Self {
        Self {
            id,
            data: HashMap::new(),
            last_used: Utc::now(),
        }
    }

    pub fn set_value(&mut self, key: &str, value: serde_json::Value) {
        self.data.insert(key.to_string(), value);
        self.last_used = Utc::now();
    }

    pub fn get_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.data
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.remove(key);
    }
}

#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        let manager = Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        };

        // Start cleanup task
        let manager_clone = manager.clone();
        tokio::spawn(async move {
            manager_clone.cleanup_loop().await;
        });

        manager
    }

    pub fn get(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(session_id).cloned()
    }

    pub fn set(&self, session_id: &str, mut session: Session) {
        session.last_used = Utc::now();
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session_id.to_string(), session);
    }

    pub fn delete(&self, session_id: &str) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(session_id);
    }

    pub fn get_or_create(&self, session_id: &str) -> Session {
        if let Some(session) = self.get(session_id) {
            return session;
        }

        let session = Session::new(session_id.to_string());
        self.set(session_id, session.clone());
        session
    }

    pub fn create_session(&self) -> String {
        let session_id = Uuid::new_v4().to_string();
        let session = Session::new(session_id.clone());
        self.set(&session_id, session);
        session_id
    }

    async fn cleanup_loop(&self) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await; // 1 hour
            self.cleanup();
        }
    }

    fn cleanup(&self) {
        let mut sessions = self.sessions.write().unwrap();
        let now = Utc::now();
        let threshold = Duration::hours(24);

        sessions.retain(|_, session| now.signed_duration_since(session.last_used) < threshold);
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
