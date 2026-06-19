#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub(crate) struct CacheEntry {
    /// データ
    pub body: Vec<u8>,
    /// キャッシュに登録した時刻
    pub registered_at: Instant,
}

pub(crate) struct InMemoryCache {
    /// キャッシュストア
    store: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// キャッシュの保持期間
    ttl: Duration,
    /// キャッシュの最大容量
    max_entries: usize,
}

impl InMemoryCache {
    /// キャッシュの初期化
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(3600),
            max_entries: 100,
        }
    }

    /// キャッシュの初期化(カスタム)
    pub fn with_config(ttl: Duration, max_entries: usize) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            ttl,
            max_entries,
        }
    }

    /// キャッシュデータの取得
    pub fn get(&self, key: &str) -> Option<CacheEntry> {
        let store = self.store.read().ok()?;
        let entry = store.get(key).cloned()?;
        if entry.registered_at.elapsed() < self.ttl {
            Some(entry)
        } else {
            None
        }
    }

    /// キャッシュデータの登録
    pub fn register(&self, key: &str, value: Vec<u8>) {
        let mut store = self
            .store
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        // キャッシュの最大容量を超える場合は、キャッシュに登録した時刻が最も古いものが削除される
        if store.len() == self.max_entries {
            if let Some(oldest_key) = store
                .iter()
                .min_by_key(|(_, value)| value.registered_at)
                .map(|(key, _)| key.to_string())
            {
                store.remove(&oldest_key);
            }
        }

        let entry = CacheEntry {
            body: value,
            registered_at: Instant::now(),
        };
        store.insert(key.to_string(), entry);
    }
}

#[cfg(test)]
mod tests {
    use crate::util::inmemory_cache::InMemoryCache;
    use std::ops::Add;
    use std::time::Duration;

    #[test]
    fn キャッシュヒット時はキャッシュされたデータを返すこと() {
        let cache = InMemoryCache::new();
        cache.register("key1", vec![1, 2, 3, 4, 5]);

        let result = cache.get("key1");
        assert!(result.is_some());
        assert_eq!(result.unwrap().body, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn キャッシュミス時は_noneを返すこと() {
        let cache = InMemoryCache::new();
        cache.register("key1", vec![4, 5, 6]);

        let result = cache.get("invalid key");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn キャッシュ保持期間を過ぎている場合は_noneを返すこと() {
        let cache = InMemoryCache::with_config(Duration::from_nanos(1), 10);
        cache.register("key1", vec![1, 3, 5, 7, 9]);

        let wait_time = cache.ttl.add(Duration::from_nanos(1));
        tokio::time::sleep(wait_time).await;

        let result = cache.get("key1");
        assert!(result.is_none());
    }

    #[test]
    fn キャッシュ最大容量を超えた場合は最古のエントリが削除されること() {
        let cache = InMemoryCache::with_config(Duration::from_secs(3600), 3);
        cache.register("key1", vec![1, 2, 3]);
        cache.register("key2", vec![4, 5, 6]);
        cache.register("key3", vec![7, 8, 9]);

        assert!(cache.get("key1").is_some());

        cache.register("key4", vec![0, 0, 0]);
        assert!(cache.get("key1").is_none());
        assert!(cache.get("key4").is_some());
    }
}
