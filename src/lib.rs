#![feature(async_fn_in_trait)]

mod api;
mod entity;
mod parser;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
struct Parser {}
