use crate::api::{Api, ApiImpl};
use crate::entity::ParseResult;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub mod api;
pub mod entity;
mod err;
pub mod parser;
mod util;

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

impl From<ParseResult> for JsValue {
    fn from(value: ParseResult) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}
