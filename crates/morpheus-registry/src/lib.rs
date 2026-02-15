use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndpointStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointRecord {
    pub id: Uuid,
    pub server: String,
    pub endpoint_url: String,
    pub api_key_ref: String,
    pub status: EndpointStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct EndpointRegistry {
    inner: Arc<RwLock<HashMap<Uuid, EndpointRecord>>>,
}

impl EndpointRegistry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(
        &self,
        server: impl Into<String>,
        endpoint_url: impl Into<String>,
        api_key_ref: impl Into<String>,
        status: EndpointStatus,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let record = EndpointRecord {
            id,
            server: server.into(),
            endpoint_url: endpoint_url.into(),
            api_key_ref: api_key_ref.into(),
            status,
            created_at: Utc::now(),
        };
        self.inner.write().insert(id, record);
        id
    }

    pub fn list_active(&self) -> Vec<EndpointRecord> {
        self.inner
            .read()
            .values()
            .filter(|r| matches!(r.status, EndpointStatus::Active))
            .cloned()
            .collect()
    }

    pub fn to_json(&self) -> serde_json::Value {
        let records: Vec<_> = self.inner.read().values().cloned().collect();
        serde_json::json!({ "endpoints": records })
    }
}
