use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("cannot fetch resource of {url}. status_code: {status_code}")]
    Network {
        url: String,
        status_code: StatusCode,
    },
    #[error("cannot deserialize response from {url}")]
    Deserialize { url: String },
}
