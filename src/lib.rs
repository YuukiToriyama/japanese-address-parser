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
