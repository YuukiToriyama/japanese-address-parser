use crate::error::ApiError;
use serde::de::DeserializeOwned;

pub struct ChimeiRuijuApiService {}

impl ChimeiRuijuApiService {
    pub async fn get<T>(&self, url: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::get(url).await.map_err(|error| ApiError::Network {
            url: url.to_string(),
            status_code: error.status().unwrap(),
        })?;
        response
            .json::<T>()
            .await
            .map_err(|_| ApiError::Deserialize {
                url: url.to_string(),
            })
    }

    pub fn get_blocking<T>(&self, url: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::blocking::get(url).map_err(|error| ApiError::Network {
            url: url.to_string(),
            status_code: error.status().unwrap(),
        })?;
        response.json::<T>().map_err(|_| ApiError::Deserialize {
            url: url.to_string(),
        })
    }
}
