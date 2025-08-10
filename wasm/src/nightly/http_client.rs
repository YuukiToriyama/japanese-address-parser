use gloo_net::http::Request;
use japanese_address_parser::http::client::ApiClient;
use japanese_address_parser::http::error::ApiClientError;
use serde::de::DeserializeOwned;

pub(crate) struct GlooNetClient {}

impl ApiClient for GlooNetClient {
    fn new() -> Self {
        GlooNetClient {}
    }

    async fn fetch<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiClientError> {
        let response = Request::get(url)
            .send()
            .await
            .map_err(|e| ApiClientError::Request {
                url: url.to_string(),
                message: e.to_string(),
            })?;
        if !response.ok() {
            return Err(ApiClientError::Request {
                url: url.to_string(),
                message: response.status_text(),
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
}
