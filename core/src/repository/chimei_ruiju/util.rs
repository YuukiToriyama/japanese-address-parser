use crate::domain::chimei_ruiju::error::ApiError;
use crate::http::error::ApiClientError;

impl From<ApiClientError> for ApiError {
    fn from(value: ApiClientError) -> Self {
        match value {
            ApiClientError::Request { url, .. } => ApiError::Network { url },
            ApiClientError::Deserialize { url, .. } => ApiError::Deserialize { url },
        }
    }
}
