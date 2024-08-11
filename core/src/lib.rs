#[cfg(all(target_family = "wasm", feature = "blocking"))]
compile_error! {
    "The `blocking` feature is not supported with wasm target."
}

pub mod api;
mod domain;
#[deprecated(since = "0.1.6", note = "This module will be deleted in v0.2")]
pub mod entity;
pub mod parser;
mod repository;
mod service;
mod tokenizer;
mod util;
