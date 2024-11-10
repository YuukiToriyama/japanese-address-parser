# @toriyama/japanese-address-parser

A Library for processing addresses of Japan written in Rust.

[![npmjs](https://img.shields.io/npm/v/%40toriyama/japanese-address-parser)](https://www.npmjs.com/package/@toriyama/japanese-address-parser)
[![install size](https://packagephobia.com/badge?p=@toriyama/japanese-address-parser)](https://packagephobia.com/result?p=@toriyama/japanese-address-parser)
[![downloads](https://img.shields.io/npm/dm/@toriyama/japanese-address-parser.svg)](https://npmcharts.com/compare/@toriyama/japanese-address-parser?minimal=true)

## Install

Install with npm:

```bash
npm install @toriyama/japanese-address-parser
```

Install with yarn:

```bash
yarn add @toriyama/japanese-address-parser
```

## Introduction

`@toriyama/japanese-address-parser` is a library for parsing Japanese addresses.  
You can split an address string into prefectures(都道府県), municipalities(市区町村), towns and villages(町村),
and each subsequent element.

This library is a JavaScript binding for [`japanese-address-parser`](https://crates.io/crates/japanese-address-parser)
crate written in Rust by using wasm-pack.
Node.js is not yet supported. If you are eager to use this library on Node.js,
please write comments on [#128](https://github.com/YuukiToriyama/japanese-address-parser/issues/128) or pull-request!

## Demo

You can try it out on the demo pages below.

- https://yuukitoriyama.github.io/japanese-address-parser/public/index.html
- https://yuukitoriyama.github.io/japanese-address-parser/public/nightly.html (include experimental feature)

## Example

```javascript
import init, {Parser} from "@toriyama/japanese-address-parser"

init().then(() => {
    const parser = new Parser()
    parser.parse("東京都千代田区丸ノ内1-1-1").then(parseResult => {
        console.log(JSON.stringify(parseResult, null, "\t"))
    })
})
```

```json
{
  "address": {
    "prefecture": "東京都",
    "city": "千代田区",
    "town": "丸の内一丁目",
    "rest": "1-1"
  }
}
```

## How it works

The input string is basically read in order from the beginning.  
Once the name of prefecture has been scanned, the names of city will be scanned, then the names of town and so on.  
We don't have the list of city names or town names in this library, but fetch them via the internet each time.  
Version 0.1 use [Geolonia住所データ](https://github.com/geolonia/japanese-addresses) authored
by [Geolonia Inc](https://www.geolonia.com/company/).  
Detection place names may fail in some cases, such as when there is a notation distortion or when county names are
omitted. In such cases, this library tries fuzzy match instead of exact match.
For more details, please visit [our repository](https://github.com/YuukiToriyama/japanese-address-parser).

## Contributing

If you want to contribute this library, please read
the [contribution guide](https://github.com/YuukiToriyama/japanese-address-parser/blob/main/CONTRIBUTING.md) to learn
how to propose bug fixes and improvements.

## License

This library is distributed under the terms of the MIT license.

## Related projects

- [@geolonia/normalize-japanese-addresses](https://www.npmjs.com/package/@geolonia/normalize-japanese-addresses)
