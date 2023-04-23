FROM jo3mccain/trunkrs as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN trunk build --release

FROM builder as development

EXPOSE 8080

ENTRYPOINT [ "trunk" ]
CMD [ "serve" ]

FROM nginx:alpine-slim as production

COPY --from=builder /workspace/dist /usr/share/nginx/html