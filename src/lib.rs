use crate::api::{Api, ApiImpl};
use crate::entity::ParseResult;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub mod api;
pub mod entity;
mod err;
pub mod parser;
mod util;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPE: &'static str = r#"
export interface ParseResult {
    address: Address;
    error: Error | undefined;
}
export interface Address {
    prefecture: string;
    city: string;
    town: string;
    rest: string;
}
export interface Error {
    error_type: string;
    error_message: string;
}
export class Parser {
  free(): void;
  constructor();
  /**
  * @param {string} address
  * @returns {Promise<ParseResult>}
  */
  parse(address: string): Promise<ParseResult>;
}"#;

#[wasm_bindgen(skip_typescript)]
pub struct Parser();

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Parser {}
    }

    pub async fn parse(&self, address: &str) -> JsValue {
        #[cfg(feature = "debug")]
        console_error_panic_hook::set_once();
        let api = ApiImpl::new();
        let result = parser::parse(api, address).await;
        serde_wasm_bindgen::to_value(&result).unwrap()
    }
}

impl From<ParseResult> for JsValue {
    fn from(value: ParseResult) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::entity::{Address, ParseResult};
    use crate::err::{Error, ParseErrorKind};
    use crate::Parser;

    #[tokio::test]
    async fn parse_成功_実在する住所() {
        let parser = Parser();
        assert_eq!(
            parser.parse("岩手県盛岡市内丸10番1号").await,
            ParseResult {
                address: Address::new("岩手県", "盛岡市", "内丸", "10番1号"),
                error: None,
            }
        )
    }

    #[tokio::test]
    async fn parse_失敗_実在しない町名() {
        let parser = Parser();
        assert_eq!(
            parser.parse("東京都中央区銀座九丁目").await,
            ParseResult {
                address: Address::new("東京都", "中央区", "", "銀座九丁目"),
                error: Some(Error::new_parse_error(ParseErrorKind::Town)),
            }
        )
    }
}
