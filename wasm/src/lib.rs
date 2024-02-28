use japanese_address_parser::api::{Api, ApiImpl};
use japanese_address_parser::entity::ParseResult;
use japanese_address_parser::parser;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Parser();

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Parser {}
    }

    pub async fn parse(&self, address: &str) -> ParseResult {
        #[cfg(feature = "debug")]
        console_error_panic_hook::set_once();
        let api = ApiImpl::new();
        parser::parse(api, address).await
    }
}
