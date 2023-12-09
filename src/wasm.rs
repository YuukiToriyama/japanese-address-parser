use crate::api::client::ApiImpl;
use crate::entity::ParseResult;
use crate::parser;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct Parser();

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Parser {}
    }

    pub async fn parse(&self, address: &str) -> ParseResult {
        console_error_panic_hook::set_once();
        let api = ApiImpl {};
        parser::parse(api, address).await
    }
}

impl From<ParseResult> for JsValue {
    fn from(value: ParseResult) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::entity::{Address, ParseResult};
    use crate::err::{Error, ParseErrorKind};
    use crate::wasm::Parser;

    #[tokio::test]
    async fn parse_success() {
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
    async fn parse_fail_unknown_town_name() {
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
