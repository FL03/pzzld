FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release -v --workspace

FROM debian:buster-slim as runner-base

RUN apt-get update -y && apt-get upgrade -y 

FROM runner-base as runner

ENV CLIENT_ID="" \
    CLIENT_SECRET="" \
    RUST_LOG="info" \
    SERVER_PORT=8080

COPY Puzzled.toml ./config/Puzzled.toml
VOLUME ["/config"]

COPY --from=builder /app/target/release/pzzld /bin/pzzld

FROM runner

EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "pzzld" ]
CMD [ "system", "--up" ]
