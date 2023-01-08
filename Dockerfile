FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

RUN rustup default nightly \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly \
    npm install -g wasm-pack

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN wasm-pack build pzzld --release

FROM scratch

COPY --chown=55 .config /config
VOLUME ["/config"]

RUN mkdir data
VOLUME ["/data"]

COPY --from=builder /workspace/pzzld/pkg /app
VOLUME [ "/app" ]

WORKDIR /app
