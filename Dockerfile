FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --color always --release --verbose -p flow

FROM debian:buster-slim as runner-base

RUN apt-get update -y && apt-get upgrade -y 

FROM runner-base as runner

ENV CLIENT_ID="" \
    CLIENT_SECRET="" \
    RUST_LOG="info" \
    SERVER_PORT=9090

COPY Puzzled.toml ./config/Puzzled.toml
VOLUME ["/config"]

COPY --from=builder /app/target/release/pzzld /bin/pzzld

EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "pzzld" ]
CMD [ "system", "--up" ]
