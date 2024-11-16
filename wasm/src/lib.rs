#[cfg(feature = "nightly")]
mod nightly;

use japanese_address_parser::parser;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(start)]
fn start() {
    #[cfg(feature = "nightly")]
    console_log::init_with_level(log::Level::Trace).expect("could not initialize log");
}

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
pub struct Parser {
    parser: parser::Parser,
}

#[warn(clippy::new_without_default)]
#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        #[cfg(feature = "debug")]
        console_error_panic_hook::set_once();
        Parser {
            parser: parser::Parser::default(),
        }
    }

    pub async fn parse(&self, address: &str) -> JsValue {
        let result = self.parser.parse(address).await;
        serde_wasm_bindgen::to_value(&result).unwrap()
    }
}
