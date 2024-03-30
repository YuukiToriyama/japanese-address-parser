# japanese-address-parser-py
A python toolkit for processing japanese addresses

<!-- pypiにパッケージをアップロードしたらバッヂを表示する -->
[![Unit test & Code check](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/code-check.yaml/badge.svg)](https://github.com/YuukiToriyama/japanese-address-parser/actions/workflows/code-check.yaml)

## What is it?
**japanese-address-parser-py** is a Python package for parsing japanese addresses.
Any address can be processed into structured data.

## Installation from PyPI
```bash
pip install japanese-address-parser-py
```

## Usage
```python
import japanese_address_parser_py

address = "神奈川県横浜市中区本町6丁目50-10"
parse_result = japanese_address_parser_py.parse(address)
print(parse_result.address) # {'rest': '50-10', 'prefecture': '神奈川県', 'town': '本町六丁目', 'city': '横浜市中区'}
print(parse_result.address["prefecture"]) # 神奈川県
print(parse_result.address["city"]) # 横浜市中区
print(parse_result.address["town"]) # 本町六丁目
print(parse_result.address["rest"]) # 50-10
```

## Development
This library is written in Rust language. You need to set up a Rust development environment to build this library.
Also, you need to install `maturin` because this library uses it in order to generate Python bindings.

```bash
# Install maturin
cargo install --locked maturin
# Clone repository
git clone https://github.com/YuukiToriyama/japanese-address-parser.git
# Build python module
cd japanse-address-parser/python
maturin build --release --out dist --find-interpreter
# Install the built library
python3 -m venv .venv
pip3 install dist/japanese_address_parser_py-[version]-cp37-abi3-[arch].whl
```

## Support

This software is maintained by [YuukiToriyama](https://github.com/yuukitoriyama).
If you have questions, please create an issue.

## Where to get source code
The source code is hosted on GitHub at:
https://github.com/YuukiToriyama/japanese-address-parser

## Acknowledgements

This software was developed inspired
by [@geolonia/normalize-japanese-addresses](https://github.com/geolonia/normalize-japanese-addresses).  
Also, the parsing process uses [Geolonia 住所データ](https://github.com/geolonia/japanese-addresses) provided
by [株式会社Geolonia](https://www.geolonia.com/company/).

## License

This crate is distributed under the terms of the MIT license.
