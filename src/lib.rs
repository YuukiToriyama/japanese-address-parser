#![feature(async_fn_in_trait)]

mod api;
mod entity;
mod parser;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::api::wasm::ApiImplForWasm;

#[wasm_bindgen]
struct Parser();

#[wasm_bindgen]
impl Parser {
    pub async fn parse(&self, address: &str) -> String {
        let api = ApiImplForWasm {};
        let result = parser::parse(api, address).await;
        serde_json::to_string(&result).unwrap()
    }
}

#[cfg(test)]
mod integration_tests {
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    use crate::Parser;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn parse_success() {
        let parser = Parser();
        assert_eq!(
            parser.parse("岩手県盛岡市内丸10番1号").await,
            "{\"prefecture\":\"岩手県\",\"city\":\"盛岡市\",\"town\":\"内丸\",\"rest\":\"10番1号\"}".to_string()
        )
    }
}