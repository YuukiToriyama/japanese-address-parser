use crate::http::client::ApiClient;
use crate::http::error::ApiClientError;
use serde::de::DeserializeOwned;

/// An implementation of `ApiClient` with `reqwest`
pub(crate) struct ReqwestApiClient {}

impl ApiClient for ReqwestApiClient {
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
