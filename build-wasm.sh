#!/bin/bash

# Ensure we're in the project root
cd "$(dirname "$0")"

# Build the WASM module
cargo build --target wasm32-wasi --release

# Create the wasm directory if it doesn't exist
mkdir -p wasm

# Copy the WASM file to the wasm directory
cp target/wasm32-wasi/release/cypher_indexer.wasm wasm/

echo "WASM module built and copied to wasm/cypher_indexer.wasm"