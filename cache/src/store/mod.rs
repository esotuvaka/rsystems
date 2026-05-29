use crate::protocol::Response;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

/// Cache entry with optional TTL
#[derive(Debug, Clone)]
struct CacheEntry {
    /// The cached value
    value: String,
    /// Unix timestamp when entry expires (0 = no expiry)
    ttl: u64,
}

impl CacheEntry {
    fn new(value: String, ttl: Option<u64>) -> Self {
        CacheEntry {
            value,
            ttl: ttl
                .map(|t| {
                    SystemTime::now()
                        .elapsed()
                        .unwrap_or(std::time::Duration::ZERO)
                        .as_secs()
                        + t
                })
                .unwrap_or(0),
        }
    }

    fn is_expired(&self, now: u64) -> bool {
        self.ttl > 0 && self.ttl < now
    }
}

/// In-memory cache store with TTL support
#[derive(Debug, Clone)]
pub struct CacheStore {
    /// Key-value store with expiration tracking
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl CacheStore {
    pub fn new() -> Self {
        CacheStore {
            entries: Arc::default(),
        }
    }

    /// Get value by key
    pub async fn get(&self, key: &str) -> Response {
        let now = SystemTime::now()
            .elapsed()
            .unwrap_or(std::time::Duration::ZERO)
            .as_secs();
        if let Some(entry) = self.entries.read().await.get(key) {
            if entry.is_expired(now) {
                // Expired, remove and return nil
                {
                    let mut entries = self.entries.write().await;
                    entries.remove(key);
                }
                return Response::Nil;
            }
            return Response::Bulk(entry.value.clone());
        }
        Response::Nil
    }

    /// Set value with optional TTL
    pub async fn set(&self, key: &str, value: &str, ttl: Option<u64>) -> Response {
        let entry = CacheEntry::new(value.to_string(), ttl);
        let mut entries = self.entries.write().await;
        entries.insert(key.to_string(), entry);
        Response::Ok("OK".to_string())
    }

    /// Delete a key
    pub async fn del(&self, key: &str) -> Response {
        let count = {
            let mut entries = self.entries.write().await;
            entries.remove(key);
            entries.len()
        };
        if key.is_empty() {
            Response::Integer(count.try_into().unwrap())
        } else {
            Response::Integer(1)
        }
    }

    /// Get all keys for debugging
    pub async fn get_keys(&self) -> Vec<String> {
        self.entries.read().await.keys().cloned().collect()
    }
}

impl Default for CacheStore {
    fn default() -> Self {
        Self::new()
    }
}
