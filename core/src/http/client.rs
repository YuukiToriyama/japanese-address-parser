use crate::http::error::ApiClientError;
use serde::de::DeserializeOwned;

/// HTTP client to fetch remote resources
///
/// 住所データマスタを取得するためのHTTPクライアントはこのトレイトを実装する必要があります。
pub(crate) trait ApiClient {
    /// Initialize `ApiClient`
    ///
    /// `ApiClient`を初期化処理を実装します。
    fn new() -> Self;

    async fn fetch<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError>;

    #[cfg(feature = "blocking")]
    fn fetch_blocking<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError>;
}
