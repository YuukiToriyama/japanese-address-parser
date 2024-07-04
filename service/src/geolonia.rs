use domain::geolonia::error::{ApiErrorKind, Error};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub struct GeoloniaApiService {}

impl GeoloniaApiService {
    pub async fn get<T>(&self, url: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::get(url)
            .await
            .map_err(|_| Error::new_api_error(ApiErrorKind::Fetch(url.to_string())))?;
        if response.status() == StatusCode::OK {
            let json = response
                .json::<T>()
                .await
                .map_err(|_| Error::new_api_error(ApiErrorKind::Deserialize(url.to_string())))?;
            return Ok(json);
        }
        Err(Error::new_api_error(ApiErrorKind::Fetch(url.to_string())))
    }
}

impl GeoloniaApiService {
    pub fn get_blocking<T>(&self, url: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let response = reqwest::blocking::get(url)
            .map_err(|_| Error::new_api_error(ApiErrorKind::Fetch(url.to_string())))?;
        if response.status() == StatusCode::OK {
            let json = response
                .json::<T>()
                .map_err(|_| Error::new_api_error(ApiErrorKind::Deserialize(url.to_string())))?;
            return Ok(json);
        }
        Err(Error::new_api_error(ApiErrorKind::Fetch(url.to_string())))
    }
}
