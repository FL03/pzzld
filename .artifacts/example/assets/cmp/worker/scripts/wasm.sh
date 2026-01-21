#! /bin/bash

cargo run build --target wasm32-unknown-unknown "$@"
cp target/wasm32-unknown-unknown/debug/worker.wasm assets/cmp/worker/worker.wasm ||
    cp target/wasm32-unknown-unknown/release/worker.wasm assets/cmp/worker/worker.wasm ||
        exit 1