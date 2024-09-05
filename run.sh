#!/bin/bash

cargo build --release

RUST_LOG=info ./target/release/cypher-indexer