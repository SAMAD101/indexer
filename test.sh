#!/bin/bash
cargo test

cargo test --package cypher-indexer --lib processing
cargo test --package cypher-indexer --lib storage

RUST_LOG=debug cargo test