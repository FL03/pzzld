FROM scratch as cached

COPY --chown=55 .config config
VOLUME ["/config"]

RUN mkdir data
VOLUME ["/data"]

COPY --chown=55 . /workspace
VOLUME ["/workspace"]

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

RUN apt-get install -y libssl-dev protobuf-compiler

FROM runner-base as runner

ENV CLIENT_ID="" \
    CLIENT_SECRET="" \
    RUST_LOG="info" \
    SERVER_PORT=8080

COPY --chown=55 --from=cached config config
VOLUME ["/config"]

COPY --from=builder /app/target/release/pzzld /bin/pzzld

FROM runner

EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "pzzld" ]
CMD [ "system", "--up" ]
