# cloud-node-discovery-rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/cloud-node-discovery.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/cloud-node-discovery)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/cloud-node-discovery?style=for-the-badge&logo=docs.rs&label=docs.rs&labelColor=555555" height="20">](https://docs.rs/cloud-node-discovery)


A Rust library for discovering nodes in cloud environments.

Work in progress.

## Features

- Providers are supported via features. You don't have to enable them all.
- By default, no providers are enabled.
- Supported providers:
  - AWS
  - UpCloud

## Example

```bash
cargo run --example aws
cargo run --example upcloud
```

See the [examples](examples) directory for more.

## License

MIT
