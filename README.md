# japanese-address-parser

[![Docs](https://docs.rs/japanese-address-parser/badge.svg)](https://docs.rs/japanese-address-parser)
[![Crates.io (latest)](https://img.shields.io/crates/v/japanese-address-parser)](https://crates.io/crates/japanese-address-parser)
![Rust Version](https://img.shields.io/badge/rust%20version-%3E%3D1.75.0-orange)
[![Unit test & Code format check](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/rust.yaml/badge.svg?branch=main)](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/rust.yaml)

A Rust Library to parse japanses addresses.

## Usage

Add this to your `Cargo.toml`

```bash
cargo add japanese-address-parser
```

### Async Api

```rust
use japanese_address_parser::api::client::ApiImpl;
use japanese_address_parser::parser::parse;

#[tokio::main]
async fn main() {
    let async_api = ApiImpl {};
    let parse_result = parse(async_api, "東京都千代田区丸の内1-1-1").await;
    println!("{:?}", parse_result);
}
```

### Blocking Api

```rust
use japanese_address_parser::api::blocking::Client;
use japanese_address_parser::parser::parse_blocking;

fn main() {
    let blocking_api = Client {};
    let parse_result = parse_blocking(blocking_api, "東京都千代田区丸の内1-1-1");
    println!("{:?}", parse_result);
}
```

## Wasm support

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