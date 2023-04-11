# tolgee-puller-rs

A CLI that pulls your Tolgee resources. Basically [tolgee-puller](https://github.com/DutchConcepts/tolgee-puller) but written in Rust.

> <sub>**Warning**
>  I'm new to Rust. Don't expect this to be of good quality.</sup>

## Getting started

This crate is not published to any package manager. It can be ran using `cargo run` or by making `tolgee-puller-rs` available by adding the `target/release` directory to your system path environment variable.

## Usage

```console
tolgee-puller-rs [OPTIONS] --lng <LNG> --ns <NS> --output-path <OUTPUT_PATH>

Options:
      --api-key <API_KEY>
      --api-url <API_URL>          Defaults to the Tolgee API
      --lng <LNG>                  Comma separated
      --ns <NS>                    Comma separated
      --output-path <OUTPUT_PATH>
      --split
  -h, --help                       Print help
  -V, --version                    Print version
```

If the following environment variables are set the command arguments will be overwritten if specified.

```console
TOLGEE_API_KEY
TOLGEE_API_URL
```

## License

Distributed under the MIT License. See `LICENSE` for more information.
