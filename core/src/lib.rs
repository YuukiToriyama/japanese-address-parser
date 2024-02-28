use crate::entity::ParseResult;
use wasm_bindgen::JsValue;

pub mod api;
pub mod entity;
mod err;
pub mod parser;
mod util;

impl From<ParseResult> for JsValue {
    fn from(value: ParseResult) -> Self {
        serde_wasm_bindgen::to_value(&value).unwrap()
    }
}
