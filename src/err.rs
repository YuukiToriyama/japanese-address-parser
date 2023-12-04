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
    pub fn new_api_error(api_error_kind: ApiErrorKind) -> Self {
        let error_message = match api_error_kind {
            ApiErrorKind::Fetch(url) => format!("{}を取得できませんでした", url),
            ApiErrorKind::Deserialize(url) => format!("{}のデシリアライズに失敗しました", url),
        };
        Error {
            error_type: "ApiError".to_string(),
            error_message,
        }
    }
}

pub enum ParseErrorKind {
    Prefecture,
    City,
    Town,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let label = match *self {
            Self::Prefecture => "都道府県",
            Self::City => "市区町村",
            Self::Town => "町名",
        };
        write!(f, "一致する{}がありませんでした", label)
    }
}

pub enum ApiErrorKind {
    Fetch(String),
    Deserialize(String),
}
