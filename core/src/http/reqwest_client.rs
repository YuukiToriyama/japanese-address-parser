use crate::http::client::ApiClient;
use crate::http::error::ApiClientError;
use serde::de::DeserializeOwned;

/// An implementation of `ApiClient` with `reqwest`
pub(crate) struct ReqwestApiClient {}

impl ApiClient for ReqwestApiClient {
    fn new() -> Self {
        ReqwestApiClient {}
    }

    async fn fetch<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| ApiClientError::Request {
                url: url.to_string(),
                message: e.to_string(),
            })?;
        let status = response.status();
        if !status.is_success() {
            return Err(ApiClientError::Request {
                url: url.to_string(),
                message: status.to_string(),
            });
        }

        response
            .json::<T>()
            .await
            .map_err(|e| ApiClientError::Deserialize {
                url: url.to_string(),
                message: e.to_string(),
            })
    }

    #[cfg(feature = "blocking")]
    fn fetch_blocking<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError> {
        let response = reqwest::blocking::get(url).map_err(|e| ApiClientError::Request {
            url: url.to_string(),
            message: e.to_string(),
        })?;

        let status = response.status();
        if !status.is_success() {
            return Err(ApiClientError::Request {
                url: url.to_string(),
                message: status.to_string(),
            });
        }

        response
            .json::<T>()
            .map_err(|e| ApiClientError::Deserialize {
                url: url.to_string(),
                message: e.to_string(),
            })
    }
}

#[cfg(all(test, feature = "experimental", not(target_arch = "wasm32")))]
mod async_tests {
    use crate::domain::chimei_ruiju::entity::PrefectureMaster;
    use crate::http::client::ApiClient;
    use crate::http::reqwest_client::ReqwestApiClient;

    #[tokio::test]
    async fn 不正なurlを渡した場合_requestエラーになる() {
        let invalid_url = "htttp://chimei-ruiju.org";
        let api_client = ReqwestApiClient {};

        let result = api_client.fetch::<PrefectureMaster>(invalid_url).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is_request());
    }

    #[tokio::test]
    async fn レスポンスが404の場合_requestエラーになる() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(404)
            .create_async()
            .await;

        let api_client = ReqwestApiClient {};
        let result = api_client.fetch::<PrefectureMaster>(&url).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is_request());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn デシリアライズに失敗した場合_deserializeエラーになる() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"hoge": true, "piyo": 100}"#)
            .create_async()
            .await;

        let api_client = ReqwestApiClient {};
        let result = api_client.fetch::<PrefectureMaster>(&url).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is_deserialize());

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn 通信にもデシリアライズにも成功した場合_データを返す() {
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "新浜県", "cities": ["新浜市"], "coordinate": {"latitude": 34.6570413, "longitude": 135.2741341}}"#)
            .create_async()
            .await;

        let api_client = ReqwestApiClient {};
        let result = api_client.fetch::<PrefectureMaster>(&url).await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.name, "新浜県");
        assert_eq!(data.cities, vec!["新浜市"]);
        assert_eq!(data.coordinate.latitude, 34.6570413);
        assert_eq!(data.coordinate.longitude, 135.2741341);

        mock.assert_async().await;
    }
}

#[cfg(all(
    test,
    feature = "blocking",
    feature = "experimental",
    not(target_arch = "wasm32")
))]
mod blocking_tests {
    use crate::domain::chimei_ruiju::entity::PrefectureMaster;
    use crate::http::client::ApiClient;
    use crate::http::reqwest_client::ReqwestApiClient;

    #[test]
    fn 不正なurlを渡した場合_requestエラーになる() {
        let invalid_url = "htttp://chimei-ruiju.org";
        let api_client = ReqwestApiClient {};
        let result = api_client.fetch_blocking::<PrefectureMaster>(invalid_url);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_request());
    }

    #[test]
    fn レスポンスが404の場合_requestエラーになる() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server.mock("GET", "/master.json").with_status(404).create();

        let api_client = ReqwestApiClient {};
        let result = api_client.fetch_blocking::<PrefectureMaster>(&url);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_request());

        mock.assert();
    }

    #[test]
    fn デシリアライズに失敗した場合_deserializeエラーになる() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"hoge": true, "piyo": 100}"#)
            .create();

        let api_client = ReqwestApiClient {};
        let result = api_client.fetch_blocking::<PrefectureMaster>(&url);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_deserialize());

        mock.assert();
    }

    #[test]
    fn 通信にもデシリアライズにも成功した場合_データを返す() {
        let mut server = mockito::Server::new();
        let url = format!("{}/master.json", &server.url());
        let mock = server
            .mock("GET", "/master.json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"name": "新浜県", "cities": ["新浜市"], "coordinate": {"latitude": 34.6570413, "longitude": 135.2741341}}"#)
            .create();

        let api_client = ReqwestApiClient {};
        let result = api_client.fetch_blocking::<PrefectureMaster>(&url);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.name, "新浜県");
        assert_eq!(data.cities, vec!["新浜市"]);
        assert_eq!(data.coordinate.latitude, 34.6570413);
        assert_eq!(data.coordinate.longitude, 135.2741341);

        mock.assert();
    }
}
