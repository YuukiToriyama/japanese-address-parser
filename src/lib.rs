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
