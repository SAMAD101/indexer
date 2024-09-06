#!/bin/bash

cd "$(dirname "$0")"

./build-wasm.sh

if ! pgrep -x "ipfs" > /dev/null
then
    ipfs daemon &
    IPFS_PID=$!
    echo "Started IPFS daemon"
else
    echo "IPFS daemon is already running"
fi

echo "Starting Cypher-Indexer..."
cargo run --release

if [ ! -z "$IPFS_PID" ]
then
    kill $IPFS_PID
    echo "Stopped IPFS daemon"
fi