#!/bin/bash

source .env

if ! pgrep -x "ipfs" > /dev/null
then
    ipfs daemon &
    IPFS_PID=$!
fi

wasmer run $WASM_MODULE_PATH --env-file .env

if [ "$BACKUP_ENABLED" = true ] && [ "$BACKUP_IPFS_PIN" = true ]
then
    BACKUP_HASH=$(ipfs add -r $BACKUP_PATH | tail -n 1 | cut -d ' ' -f 2)
    echo "Backup pinned to IPFS with hash: $BACKUP_HASH"
fi

if [ ! -z "$IPFS_PID" ]
then
    kill $IPFS_PID
fi