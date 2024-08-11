# japanese-address-parser

[![Docs](https://docs.rs/japanese-address-parser/badge.svg)](https://docs.rs/japanese-address-parser)
[![Crates.io (latest)](https://img.shields.io/crates/v/japanese-address-parser)](https://crates.io/crates/japanese-address-parser)
![Rust Version](https://img.shields.io/badge/rust%20version-%3E%3D1.73.0-orange)
[![Unit test & Integration test](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/run-test.yaml/badge.svg?branch=main)](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/run-test.yaml)

A Rust Library to parse japanese addresses.

## Usage

Add this to your `Cargo.toml`

```bash
cargo add japanese-address-parser
# or
cargo add japanese-address-parser -F blocking
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
Pre-compiled wasm module is available npmjs.com

```bash
npm install @toriyama/japanese-address-parser
```

```javascript
import init, {Parser} from "@toriyama/japanese-address-parser"

init().then(() => {
    const parser = new Parser()
    parser.parse("東京都千代田区丸の内1-1-1").then(parseResult => {
        console.log(JSON.stringify(parseResult, null, "\t"))
    })
})
```

## Python support(experimental)

[![PyPI - Version](https://img.shields.io/pypi/v/japanese-address-parser-py)](https://pypi.org/project/japanese-address-parser-py/)

This library can be called from Python world. For more detail, see [python module's README](python/README.md).

## Road to v1

The goals that this library aims to achieve are below.

- Supports not only wasm target but also various platforms and architectures.
- Enables more advanced normalization. For example, provides more detailed analysis than town level.
- Provides latlng of the given address.
- Enables processing of town names that have ceased to exist as a result of municipal mergers.

## Support

This software is maintained by [YuukiToriyama](https://github.com/yuukitoriyama).
If you have questions, please create an issue.

## Acknowledgements

This software was developed inspired
by [@geolonia/normalize-japanese-addresses](https://github.com/geolonia/normalize-japanese-addresses).  
Also, the parsing process uses [Geolonia 住所データ](https://github.com/geolonia/japanese-addresses) provided
by [株式会社Geolonia](https://www.geolonia.com/company/).

## License

This crate is distributed under the terms of the MIT license.
