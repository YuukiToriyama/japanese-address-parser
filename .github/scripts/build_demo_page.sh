#!/bin/sh
npm install -g wasm-pack
cd wasm
wasm-pack build --target web --scope toriyama --out-name japanese_address_parser --features debug
cd ../
mkdir dist
rm wasm/pkg/.gitignore
mv wasm/pkg dist
mv public dist
