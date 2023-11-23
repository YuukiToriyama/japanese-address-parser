use std::fmt::{Display, Formatter};

use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct Error {
    pub error_type: String,
    pub error_message: String,
}

impl Error {
    pub fn new_parse_error(parse_error_kind: ParseErrorKind) -> Self {
        Error {
            error_type: "ParseError".to_string(),
            error_message: parse_error_kind.to_string(),
        }
    }
    pub fn new_api_error(message: &str) -> Self {
        Error {
            error_type: "ResourceUnavailableError".to_string(),
            error_message: message.to_string(),
        }
    }
}

pub enum ParseErrorKind {
    PREFECTURE,
    CITY,
    TOWN,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match *self {
            Self::PREFECTURE => "都道府県",
            Self::CITY => "市区町村",
            Self::TOWN => "町名",
        };
        write!(f, "一致する{}がありませんでした", label)
    }
}
