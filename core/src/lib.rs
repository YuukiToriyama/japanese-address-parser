//! A Rust library to parse japanese addresses.
//!
//! ## Feature flags
//! - `blocking`: Provide method that works synchronously
//! - `city-name-correction`*(enabled by default)*: Enable autocorrection if ambiguous city name was typed
//! - `format-house-number`: Enable normalization of addresses after town name
//! - `eliminate-whitespaces`*(experimental)*: Enable elimination of whitespaces from given text
//! - `experimental`: Enable experimental module

#![cfg_attr(docsrs, feature(doc_cfg))]
#[cfg(all(target_family = "wasm", feature = "blocking"))]
compile_error! {
    "The `blocking` feature is not supported with wasm target."
}

mod adapter;
pub mod domain;
#[cfg(feature = "experimental")]
#[cfg_attr(docsrs, doc(cfg(feature = "experimental")))]
pub mod experimental;
mod formatter;
mod interactor;
pub mod parser;
mod repository;
mod service;
mod tokenizer;
mod util;
