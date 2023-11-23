#![feature(async_fn_in_trait)]

mod api;
mod entity;
mod err;
mod parser;

use crate::api::wasm::ApiImplForWasm;
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
        let api = ApiImplForWasm {};
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
            "{\"prefecture\":\"岩手県\",\"city\":\"盛岡市\",\"town\":\"内丸\",\"rest\":\"10番1号\"}".to_string()
        )
    }
}
