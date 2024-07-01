use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::error::ApiError;

pub struct ChimeiRuijuApiService {}

impl ChimeiRuijuApiService {
    pub async fn get<T>(&self, url: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::get(url).await.map_err(|_| ApiError::Network {
            url: url.to_string(),
        })?;
        if response.status() == StatusCode::NOT_FOUND {
            return Err(ApiError::NotFound {
                url: url.to_string(),
            });
        }
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
        let response = reqwest::blocking::get(url).map_err(|_| ApiError::Network {
            url: url.to_string(),
        })?;
        if response.status() == StatusCode::NOT_FOUND {
            return Err(ApiError::NotFound {
                url: url.to_string(),
            });
        }
        response.json::<T>().map_err(|_| ApiError::Deserialize {
            url: url.to_string(),
        })
    }
}
