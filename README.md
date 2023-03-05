# `flipperzero-template`🚀

A template for kick-starting a Rust + FlipperZero project using [`flipperzero-rs`](https://github.com/flipperzero-rs/flipperzero) 🐬❤️🦀.

Currently supports SDK 14.0 ([flipperzero-firmware@0.77.1](https://github.com/flipperdevices/flipperzero-firmware/tree/0.77.1)).

# Usage

## Initial setup

1. Install [`rustup`](https://rust-lang.github.io/rustup/) by following the instructions on [`rustup.rs`](https://rustup.rs/).
2. Use `rustup` to install the `thumbv7em-none-eabihf` target:
    ```
    rustup target add thumbv7em-none-eabihf
    ```

## Use `cargo generate` to clone this template.

```
cargo generate --git https://github.com/flipperzero-rs/flipperzero-template.git --name my-project
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
