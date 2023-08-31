# `flipperzero-template`🚀

A template for kick-starting a Rust + FlipperZero project using [`flipperzero-rs`](https://github.com/flipperzero-rs/flipperzero) 🐬❤️🦀.

Currently supports SDK 35.0 ([flipperzero-firmware@0.89.0](https://github.com/flipperdevices/flipperzero-firmware/tree/0.89.0)).

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

The resulting `.fap` binary can be found in [`target/thumbv7em-none-eabihf/debug`](target/thumbv7em-none-eabihf/debug).

# License

This template is licensed under the [MIT License](https://github.com/flipperzero-rs/flipperzero/blob/v0.7.2/LICENSE).
