use crate::http::error::ApiClientError;
use serde::de::DeserializeOwned;

/// HTTP client to fetch remote resources
///
/// 住所データマスタを取得するためのHTTPクライアントはこのトレイトを実装する必要があります。
#[trait_variant::make(Send)]
pub trait ApiClient {
    /// Initialize `ApiClient`
    ///
    /// `ApiClient`を初期化処理を実装します。
    fn new() -> Self;

    /// Fetches and deserializes data from a remote URL asynchronously
    ///
    /// 引数で指定したURLから非同期的にデータを取得し、デシリアライズする処理を実装します。
    async fn fetch<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError>;

    /// Fetches and deserializes data from a remote URL synchronously
    ///
    /// 引数で指定したURLから同期的にデータを取得し、デシリアライズする処理を実装します。
    #[cfg(feature = "blocking")]
    fn fetch_blocking<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError>;
}
