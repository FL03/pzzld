FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly && \
    cargo install trunk wasm-bindgen-cli

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN trunk build

FROM scratch as wasm

COPY --chown=55 .config /config
VOLUME ["/config"]

COPY --from=builder /workspace/dist /app
VOLUME [ "/app" ]

WORKDIR /app

FROM builder

EXPOSE 8080

ENTRYPOINT [ "trunk" ]
CMD [ "serve" ]