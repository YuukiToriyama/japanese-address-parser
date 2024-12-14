# japanese-address-parser

[![Docs](https://docs.rs/japanese-address-parser/badge.svg)](https://docs.rs/japanese-address-parser)
[![Crates.io (latest)](https://img.shields.io/crates/v/japanese-address-parser)](https://crates.io/crates/japanese-address-parser)
![Rust Version](https://img.shields.io/badge/rust%20version-%3E%3D1.75.0-orange)
[![Unit test & Integration test](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/run-test.yaml/badge.svg?branch=main)](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/run-test.yaml)

A Rust library for parsing Japanese addresses.

## Usage

Add the following to your `Cargo.toml`.

```toml
[dependencies]
japanese-address-parser = "0.2"
```

### Async Version

```rust
use japanese_address_parser::parser::Parser;

#[tokio::main]
async fn main() {
    let parser: Parser = Default::default();
    let parse_result = parser.parse("東京都千代田区丸の内1-1-1").await;
    println!("{:?}", parse_result);
}
```

### Blocking Version

```rust
use japanese_address_parser::parser::Parser;

fn main() {
    let parser: Parser = Default::default();
    let parse_result = parser.parse_blocking("東京都千代田区丸の内1-1-1"); // `parse_blocking()` is available on `blocking` feature only
    println!("{:?}", parse_result);
}
```

## Wasm support

[![npmjs](https://img.shields.io/npm/v/%40toriyama/japanese-address-parser)](https://www.npmjs.com/package/@toriyama/japanese-address-parser)

This crate is designed to be buildable for `wasm32-unknown-unknown` with `wasm-pack`.
Pre-compiled wasm module is available on npmjs.com

You can run this crate on your browser. For more details, see [wasm module's README](wasm/README.md).

## Python support(experimental)

[![PyPI - Version](https://img.shields.io/pypi/v/japanese-address-parser-py)](https://pypi.org/project/japanese-address-parser-py/)

This library can be called from the Python world. For more details, see [python module's README](python/README.md).

## Road to v1

The goals of this library are as follows.

- Supports not only wasm but also multiple platforms and architectures.
- Enables more advanced normalization. For example, provides more detailed analysis than town level.
- Returns the location of the given address.
- Enables processing of town names that no longer exist due to municipal mergers.

## Support

This software is maintained by [YuukiToriyama](https://github.com/yuukitoriyama).
If you have any questions, please create a new issue.

## Acknowledgements

This software was inspired
by [@geolonia/normalize-japanese-addresses](https://github.com/geolonia/normalize-japanese-addresses).  
In addition, the parsing process uses [Geolonia 住所データ](https://github.com/geolonia/japanese-addresses) which is
provided by [株式会社Geolonia](https://www.geolonia.com/company/).

## License

This crate is distributed under the terms of the MIT license.
