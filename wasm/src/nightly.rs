mod http_client;

use crate::nightly::http_client::GlooNetClient;
use japanese_address_parser::experimental::parser::{DataSource, Parser, ParserOptions};
use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPE: &'static str = r#"
export function parse_experimental(
    address: string,
    options: ParseOptions
): Promise<ParsedAddress>;

export interface ParseOptions {
    dataSource: "chimeiruiju" | "geolonia";
    correctIncompleteCityNames: boolean | null;
    verbose: boolean | null;
}

export interface Metadata {
    latitude: number | undefined;
    longitude: number | undefined;
    depth: number;
}

export interface ParsedAddress {
    prefecture: string;
    city: string;
    town: string;
    rest: string;
    metadata: Metadata;
}"#;

#[derive(Deserialize)]
pub struct Options {
    #[serde(alias = "dataSource")]
    data_source: String,
    #[serde(alias = "correctIncompleteCityNames")]
    correct_incomplete_city_names: Option<bool>,
    #[serde(alias = "verbose")]
    verbose: Option<bool>,
}

#[wasm_bindgen(skip_typescript, skip_jsdoc)]
pub async fn parse_experimental(address: &str, options: JsValue) -> JsValue {
    let parser_options: ParserOptions = match serde_wasm_bindgen::from_value::<Options>(options) {
        Err(error) => {
            log::warn!(
                "オプションが指定されていないか、指定されたオプションの形式が正しくありません"
            );
            log::error!("{}", error);
            ParserOptions::default()
        }
        Ok(options) => ParserOptions {
            data_source: match options.data_source.as_str() {
                "chimeiruiju" => DataSource::ChimeiRuiju,
                "geolonia" => DataSource::Geolonia,
                _ => DataSource::default(),
            },
            correct_incomplete_city_names: match options.correct_incomplete_city_names {
                Some(boolean) => boolean,
                _ => true,
            },
            verbose: match options.verbose {
                Some(boolean) => boolean,
                _ => true,
            },
        },
    };
    let parser = Parser::<GlooNetClient>::new();
    let result = parser.parse_with_options(address, &parser_options).await;
    serde_wasm_bindgen::to_value(&result).expect("could not serialize struct into json")
}
