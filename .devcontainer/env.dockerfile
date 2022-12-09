FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    protobuf-compiler

FROM base

RUN rustup update

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly
