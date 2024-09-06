#!/bin/bash

# Ensure we're in the project root
cd "$(dirname "$0")"

# Build WASM module
./build-wasm.sh

# Start IPFS daemon if not already running
if ! pgrep -x "ipfs" > /dev/null
then
    ipfs daemon &
    IPFS_PID=$!
    echo "Started IPFS daemon"
else
    echo "IPFS daemon is already running"
fi

# Run the indexer
echo "Starting Cypher-Indexer..."
cargo run --release

# Stop IPFS daemon if we started it
if [ ! -z "$IPFS_PID" ]
then
    kill $IPFS_PID
    echo "Stopped IPFS daemon"
fi