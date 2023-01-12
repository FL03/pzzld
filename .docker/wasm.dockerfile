FROM --platform=$BUILDPLATFORM rust:1.64 AS buildbase

ADD . /workspace
WORKDIR /workspace

RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y \
        git \
        clang
    rustup target add wasm32-wasi
EOT
# This line installs WasmEdge including the AOT compiler
RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

FROM buildbase AS build

COPY . .

# Build the Wasm binary
RUN --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/registry/index \
    cargo build --target wasm32-wasi --release

# This line builds the AOT Wasm binary
RUN /root/.wasmedge/bin/wasmedgec target/wasm32-wasi/release/pzzld.wasm pzzld.wasm

FROM scratch

ENTRYPOINT [ "pzzld.wasm" ]

COPY --link --from=build /workspace/pzzld.wasm /pzzld.wasm