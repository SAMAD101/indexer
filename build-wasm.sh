#!/bin/bash

cd "$(dirname "$0")"

cargo build --target wasm32-wasi --release

mkdir -p wasm

cp target/wasm32-wasi/release/cypher_indexer.wasm wasm/

echo "WASM module built and copied to wasm/cypher_indexer.wasm"