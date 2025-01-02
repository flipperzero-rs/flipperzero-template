# `flipperzero-template`🚀

A template for kick-starting a Rust + FlipperZero project using [`flipperzero-rs`](https://github.com/flipperzero-rs/flipperzero) 🐬❤️🦀.

# Usage

## Initial setup

1. Install [`rustup`](https://rust-lang.github.io/rustup/) by following the instructions on [`rustup.rs`](https://rustup.rs/).
1. Install the nightly build tool-chain to support the[`different-binary-name`](https://doc.rust-lang.org/cargo/reference/unstable.html#different-binary-name) feature:
    ```
    rustup toolchain install nightly
    ```
1. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate):
    ```
    cargo install cargo-generate
    ```
1. Use `rustup` to install the `thumbv7em-none-eabihf` target to the nightly build:
    ```
    rustup target add --toolchain nightly thumbv7em-none-eabihf
    ```

## Generate the project
1. Use `cargo generate` to clone this template:
    ```
    cargo generate --git https://github.com/flipperzero-rs/flipperzero-template.git --name my-project
    ```
1. Switch into the local directory:
    ```
    cd my-project
    ```

## Build with `cargo build`

```
cargo build
```

## Copy the binary to your Flipper Zero

> [!IMPORTANT]
> This requires the `storage` command from [`flipperzero-tools`](https://crates.io/crates/flipperzero-tools) (`cargo install --locked flipperzero-tools`) or [`storage.py`](https://github.com/flipperdevices/flipperzero-firmware/blob/dev/scripts/storage.py) from the official SDK.

The resulting `.fap` binary can be found in [`target/thumbv7em-none-eabihf/debug`](target/thumbv7em-none-eabihf/debug).

```sh
storage send target/thumbv7em-none-eabihf/release/my-project.fap /ext/apps/Examples/my-project.fap
```

## Build and run on change

> [!IMPORTANT]
> This requires the `run-fap` command from [`flipperzero-tools`](https://crates.io/crates/flipperzero-tools) (`cargo install --locked flipperzero-tools`) or [`runfap.py`](https://github.com/flipperdevices/flipperzero-firmware/blob/dev/scripts/runfap.py) from the official SDK.

You can automatically build and run your binary using [`cargo-watch`](https://crates.io/crates/cargo-watch) and the `run-fap` tool.

```sh
cargo watch -s 'cargo build --release && run-fap target/thumbv7em-none-eabihf/release/my-project.fap'
```

# License

This template is licensed under the [MIT License](https://github.com/flipperzero-rs/flipperzero/blob/v0.7.2/LICENSE).
