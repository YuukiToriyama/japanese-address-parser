# japanese-address-parser

[![Docs](https://docs.rs/japanese-address-parser/badge.svg)](https://docs.rs/japanese-address-parser)
[![Crates.io (latest)](https://img.shields.io/crates/v/japanese-address-parser)](https://crates.io/crates/japanese-address-parser)
![Rust Version](https://img.shields.io/badge/rust%20version-%3E%3D1.64.0-orange)
[![Unit test & Code check](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/code-check.yaml/badge.svg)](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/code-check.yaml)

A Rust Library to parse japanese addresses.

## Usage

Add this to your `Cargo.toml`

```bash
cargo add japanese-address-parser
```

### Async Api

```rust
use japanese_address_parser::api::{Api, ApiImpl};
use japanese_address_parser::parser::parse;

#[tokio::main]
async fn main() {
    let async_api = ApiImpl::new();
    let parse_result = parse(async_api, "東京都千代田区丸の内1-1-1").await;
    println!("{:?}", parse_result);
}
```

### Blocking Api

```rust
use japanese_address_parser::api::{BlockingApi, BlockingApiImpl};
use japanese_address_parser::parser::parse_blocking;

fn main() {
    let blocking_api = BlockingApiImpl::new();
    let parse_result = parse_blocking(blocking_api, "東京都千代田区丸の内1-1-1");
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
