use crate::http::client::ApiClient;
use crate::http::error::ApiClientError;
use crate::util::inmemory_cache::InMemoryCache;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

/// Wrapper of `ApiClient` that enables in-memory cache
///
/// ```rust
/// use japanese_address_parser::http::cached_client::CachedApiClient;
/// use japanese_address_parser::http::client::ApiClient;
/// use japanese_address_parser::http::reqwest_client::ReqwestApiClient;
///
/// let client = CachedApiClient::<ReqwestApiClient>::new();
/// ```
pub struct CachedApiClient<C: ApiClient> {
    client: C,
    cache: InMemoryCache,
}

impl<C: ApiClient> CachedApiClient<C> {
    #[allow(dead_code)]
    fn with_config(ttl: Duration, max_entries: usize) -> Self {
        Self {
            client: C::new(),
            cache: InMemoryCache::with_config(ttl, max_entries),
        }
    }
}

impl<C: ApiClient + Sync> ApiClient for CachedApiClient<C> {
    fn new() -> Self {
        Self {
            client: C::new(),
            cache: InMemoryCache::new(),
        }
    }

    async fn fetch<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError> {
        // キャッシュが利用できる場合は、キャッシュからバイト列を取得してデシリアライズして利用する
        if let Some(entry) = self.cache.get(url) {
            return serde_json::from_slice::<T>(&entry.body).map_err(|e| {
                ApiClientError::Deserialize {
                    url: url.to_string(),
                    message: e.to_string(),
                }
            });
        }

        // キャッシュが利用できない場合は、APIリクエストを行ないデータを取得、取得したデータをキャッシュに保存する
        let response = self.client.fetch::<Value>(url).await?;
        let bytes = serde_json::to_vec(&response).map_err(|e| ApiClientError::Deserialize {
            url: url.to_string(),
            message: e.to_string(),
        })?;
        self.cache.register(url, bytes.clone());
        serde_json::from_slice::<T>(&bytes).map_err(|e| ApiClientError::Deserialize {
            url: url.to_string(),
            message: e.to_string(),
        })
    }

    #[cfg(feature = "blocking")]
    fn fetch_blocking<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError> {
        // キャッシュが利用できる場合は、キャッシュからバイト列を取得してデシリアライズして利用する
        if let Some(entry) = self.cache.get(url) {
            return serde_json::from_slice::<T>(&entry.body).map_err(|e| {
                ApiClientError::Deserialize {
                    url: url.to_string(),
                    message: e.to_string(),
                }
            });
        }

        // キャッシュが利用できない場合は、APIリクエストを行ないデータを取得、取得したデータをキャッシュに保存する
        let response = self.client.fetch_blocking::<Value>(url)?;
        let bytes = serde_json::to_vec(&response).map_err(|e| ApiClientError::Deserialize {
            url: url.to_string(),
            message: e.to_string(),
        })?;
        self.cache.register(url, bytes.clone());
        serde_json::from_slice::<T>(&bytes).map_err(|e| ApiClientError::Deserialize {
            url: url.to_string(),
            message: e.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::http::cached_client::CachedApiClient;
    use crate::http::client::ApiClient;
    use crate::http::error::ApiClientError;
    use serde::de::DeserializeOwned;
    use serde_json::Value;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread::sleep;
    use std::time::Duration;

    struct MockApiClient {
        called_count: AtomicUsize,
    }

    impl MockApiClient {
        fn generate_dummy_response<T: DeserializeOwned>(
            &self,
            _url: &str,
        ) -> Result<T, ApiClientError> {
            Ok(serde_json::from_str(&format!(
                "{{\"called_count\": {}}}",
                self.called_count.load(Ordering::SeqCst),
            ))
            .unwrap())
        }

        fn increment(&self) {
            self.called_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl ApiClient for MockApiClient {
        fn new() -> Self {
            Self {
                called_count: 0.into(),
            }
        }

        async fn fetch<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError> {
            self.increment();
            self.generate_dummy_response(url)
        }

        #[cfg(feature = "blocking")]
        fn fetch_blocking<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError> {
            self.increment();
            self.generate_dummy_response(url)
        }
    }

    #[tokio::test]
    async fn キャッシュヒット時はキャッシュされたデータを返すこと() {
        let client = CachedApiClient::<MockApiClient>::new();
        let response = client.fetch::<Value>("/endpoint").await.unwrap();
        assert_eq!(response.get("called_count").unwrap().as_u64(), Some(1));

        let response = client.fetch::<Value>("/endpoint").await.unwrap();
        assert_eq!(response.get("called_count").unwrap().as_u64(), Some(1));
        assert_ne!(response.get("called_count").unwrap().as_u64(), Some(2));
    }

    #[tokio::test]
    async fn キャッシュミス時はfetchによるデータを返すこと() {
        let client = CachedApiClient::<MockApiClient>::with_config(Duration::from_secs(1), 10);
        let response = client.fetch::<Value>("/endpoint").await.unwrap();
        assert_eq!(response.get("called_count").unwrap().as_u64(), Some(1));

        // 1秒待機
        tokio::time::sleep(Duration::from_secs(1)).await;

        let response = client.fetch::<Value>("/endpoint").await.unwrap();
        // TTL=1秒なのでキャッシュミスになるはず
        assert_ne!(response.get("called_count").unwrap().as_u64(), Some(1));
        assert_eq!(response.get("called_count").unwrap().as_u64(), Some(2));
    }

    #[test]
    #[cfg(feature = "blocking")]
    fn キャッシュヒット時はキャッシュされたデータを返すこと_blocking() {
        let client = CachedApiClient::<MockApiClient>::new();
        let response = client.fetch_blocking::<Value>("/endpoint").unwrap();
        assert_eq!(response.get("called_count").unwrap().as_u64(), Some(1));

        let response = client.fetch_blocking::<Value>("/endpoint").unwrap();
        assert_eq!(response.get("called_count").unwrap().as_u64(), Some(1));
        assert_ne!(response.get("called_count").unwrap().as_u64(), Some(2));
    }

    #[test]
    #[cfg(feature = "blocking")]
    fn キャッシュミス時はfetchによるデータを返すこと_blocking() {
        let client = CachedApiClient::<MockApiClient>::with_config(Duration::from_secs(1), 10);
        let response = client.fetch_blocking::<Value>("/endpoint").unwrap();
        assert_eq!(response.get("called_count").unwrap().as_u64(), Some(1));

        // 1秒待機
        sleep(Duration::from_secs(1));

        let response = client.fetch_blocking::<Value>("/endpoint").unwrap();
        // TTL=1秒なのでキャッシュミスになるはず
        assert_ne!(response.get("called_count").unwrap().as_u64(), Some(1));
        assert_eq!(response.get("called_count").unwrap().as_u64(), Some(2));
    }
}
