#! /bin/bash

cargo component build --target wasm32-wasmp1 -p "$@"
