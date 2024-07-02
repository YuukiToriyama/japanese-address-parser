use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum ApiError {
    #[error("network error occurs: {url}")]
    Network { url: String },
    #[error("cannot fetch resource of {url}")]
    NotFound { url: String },
    #[error("cannot deserialize response from {url}")]
    Deserialize { url: String },
}
