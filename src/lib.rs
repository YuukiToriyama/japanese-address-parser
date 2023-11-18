#![feature(async_fn_in_trait)]

mod entity;
mod parser;
mod api;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
struct Parser {}
