#![feature(async_fn_in_trait)]

mod api;
mod entity;
mod err;
mod parser;

use crate::api::client::ApiImpl;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
struct Parser();

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Parser {}
    }

    pub async fn parse(&self, address: &str) -> String {
        console_error_panic_hook::set_once();
        let api = ApiImpl {};
        let result = parser::parse(api, address).await;
        serde_json::to_string(&result).unwrap()
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::Parser;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn parse_success() {
        let parser = Parser();
        assert_eq!(
            parser.parse("岩手県盛岡市内丸10番1号").await,
            "{\"address\":{\"prefecture\":\"岩手県\",\"city\":\"盛岡市\",\"town\":\"内丸\",\"rest\":\"10番1号\"},\"error\":null}".to_string()
        )
    }

    #[wasm_bindgen_test]
    async fn parse_fail_unknown_town_name() {
        let parser = Parser();
        assert_eq!(
            parser.parse("東京都中央区銀座九丁目").await,
            "{\"address\":{\"prefecture\":\"東京都\",\"city\":\"中央区\",\"town\":\"\",\"rest\":\"銀座九丁目\"},\"error\":{\"error_type\":\"ParseError\",\"error_message\":\"一致する町名がありませんでした\"}}".to_string()
        )
    }
}
