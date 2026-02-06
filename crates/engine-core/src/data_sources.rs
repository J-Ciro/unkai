//! Data sources: obtienen datos para widgets

use anyhow::Result;
use serde_json::Value;
use std::time::{Duration, Instant};

/// Trait para una fuente de datos
pub trait DataSource: Send + Sync {
    /// Nombre descriptivo
    fn name(&self) -> &str;

    /// Fetch data asincronamente
    fn fetch(&self) -> impl std::future::Future<Output = Result<Value>> + Send;

    /// TTL del cache
    fn cache_ttl(&self) -> Duration {
        Duration::from_secs(5)
    }
}

/// Caché para data sources
pub struct DataSourceCache {
    data: Option<Value>,
    last_fetch: Option<Instant>,
    ttl: Duration,
}

impl DataSourceCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            data: None,
            last_fetch: None,
            ttl,
        }
    }

    pub fn get_or_fetch<F>(&mut self, fetch_fn: F) -> Result<Value>
    where
        F: Fn() -> Result<Value>,
    {
        let now = Instant::now();

        // Si hay cache y no expiró, retornar
        if let Some(data) = &self.data {
            if let Some(last) = self.last_fetch {
                if now.duration_since(last) < self.ttl {
                    return Ok(data.clone());
                }
            }
        }

        // Fetch nuevo
        let new_data = fetch_fn()?;
        self.data = Some(new_data.clone());
        self.last_fetch = Some(now);

        Ok(new_data)
    }

    pub fn is_expired(&self) -> bool {
        if let Some(last) = self.last_fetch {
            Instant::now().duration_since(last) > self.ttl
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_stores_data() {
        let mut cache = DataSourceCache::new(Duration::from_secs(10));

        let result = cache.get_or_fetch(|| {
            Ok(serde_json::json!({"value": 42}))
        });

        assert!(result.is_ok());
        assert_eq!(cache.data.as_ref().unwrap()["value"], 42);
    }

    #[test]
    fn test_cache_expires() {
        let mut cache = DataSourceCache::new(Duration::from_millis(10));
        
        cache.get_or_fetch(|| Ok(serde_json::json!({"v": 1}))).ok();
        
        std::thread::sleep(Duration::from_millis(20));
        
        assert!(cache.is_expired());
    }
}
