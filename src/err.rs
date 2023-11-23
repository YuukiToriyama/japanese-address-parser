use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    pub error_type: String,
    pub error_message: String,
}

impl Error {
    pub fn new_parse_error(message: &str) -> Self {
        Error {
            error_type: "ParseError".to_string(),
            error_message: message.to_string(),
        }
    }
    pub fn new_resource_unavailable_error(message: &str) -> Self {
        Error {
            error_type: "ResourceUnavailableError".to_string(),
            error_message: message.to_string(),
        }
    }
}
