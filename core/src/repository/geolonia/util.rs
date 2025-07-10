use crate::domain::geolonia::error::{ApiErrorKind, Error};
use crate::http::error::ApiClientError;

impl From<ApiClientError> for Error {
    fn from(value: ApiClientError) -> Self {
        match value {
            ApiClientError::Request { url, .. } => {
                Error::new_api_error(ApiErrorKind::Fetch(url.to_string()))
            }
            ApiClientError::Deserialize { url, .. } => {
                Error::new_api_error(ApiErrorKind::Deserialize(url.to_string()))
            }
        }
    }
}
