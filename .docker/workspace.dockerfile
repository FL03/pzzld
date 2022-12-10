FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD .. /app
WORKDIR /app

COPY .. .
RUN cargo build --release -v --workspace

VOLUME [ "data" ]
