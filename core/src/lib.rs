//! A Rust library to parse japanese addresses.
//!
//! ## Feature flags
//! - `blocking`: Provide method that works synchronously
//! - `city-name-correction`*(enabled by default)*: Enable autocorrection if ambiguous city name was typed
//! - `format-house-number`: Enable normalization of addresses after town name

#[cfg(all(target_family = "wasm", feature = "blocking"))]
compile_error! {
    "The `blocking` feature is not supported with wasm target."
}

pub mod api;
mod domain;
#[deprecated(since = "0.1.6", note = "This module will be deleted in v0.2")]
pub mod entity;
mod formatter;
pub mod parser;
mod repository;
mod service;
mod tokenizer;
mod util;
