# pzzld

[![Clippy](https://github.com/FL03/pzzld/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/pzzld/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/pzzld/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/pzzld/actions/workflows/docker.yml)
[![Rust](https://github.com/FL03/pzzld/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/pzzld/actions/workflows/rust.yml)

***

Welcome to Puzzled, this application services my Ethereum namespace in several ways emulating several critical capabilities currently being pursued by Scattered-Systems, DAO LLC. The application is written primarily in Rust, leveraging Axum and Clap application frameworks to create a high-preformance web-application capable of serving static assets and wasm files. Additionally, the applicaiton implements a host of authentication measures

## Getting Started

### Building from the source

#### _Clone the repository_

```bash
git clone https://github.com/FL03/pzzld
```

### Docker

#### _Build the image locally_

```bash
docker buildx build --tag pzzld:latest .
```

#### _Pull the pre-built image_

```bash
docker pull jo3mccain/pzzld:latest
```

#### _Run the image_

```bash
docker run \
    -p 8080:8080 \
    -e CLIENT_ID="${CLIENT_ID}" \
    -e CLIENT_SECRET="${CLIENT_SECRET}" \
    -e S3_ACCESS_KEY="${S3_ACCESS_KEY}" \
    -e S3_SECRET_KEY="${S3_SECRET_KEY}"  \
    jo3mccain/pzzld:latest
```

### Usage

```rust

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
