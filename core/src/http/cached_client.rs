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
