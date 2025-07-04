use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("Network error: {url} {message}")]
    Request { url: String, message: String },
    #[error("Deserialization error: {message}")]
    Deserialize { url: String, message: String },
}
