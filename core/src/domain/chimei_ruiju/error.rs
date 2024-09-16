use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum ApiError {
    #[error("network error occurs: {url}")]
    Network { url: String },
    #[error("resource not found: {url}")]
    NotFound { url: String },
    #[error("deserialize error occurs: {url}")]
    Deserialize { url: String },
}
