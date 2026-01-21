# pzzld

[![crates.io](https://img.shields.io/crates/v/pzzld?style=for-the-badge&logo=rust)](https://crates.io/crates/pzzld)
[![docs.rs](https://img.shields.io/docsrs/pzzld?style=for-the-badge&logo=docs.rs)](https://docs.rs/pzzld)
[![Docker Image Version](https://img.shields.io/docker/v/jo3mccain/pzzld?style=for-the-badge&logo=docker)](https://hub.docker.com/r/jo3mccain/pzzld)
[![GitHub License](https://img.shields.io/github/license/FL03/pzzld?style=for-the-badge&logo=github)](LICENSE)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

`pzzld` is a dynamic WebAssembly platform for creating, hosting, and managing composite applications.

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies.pzzld]
features = []
version = "0.0.x"
```

### Examples

Listed below are some basic examples of how to use `pzzld`:

#### _Basic Usage_

```rust
    extern crate pzzld;

    fn main() -> Result<(), pzzld::Error> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
        tracing::info! { "Welcome to {name}", name = "pzzld" }

        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
