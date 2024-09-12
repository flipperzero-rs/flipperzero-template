//! Template project for Flipper Zero.
//! This app prints "Hello, Rust!" to the console then exits.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::ffi::CStr;

use flipperzero::println;
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
    name = "Flipper Zero Rust",
    app_version = 1,
    has_icon = true,
    // See https://github.com/flipperzero-rs/flipperzero/blob/v0.11.0/docs/icons.md for icon format
    icon = "rustacean-10x10.icon",
);

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    println!("Hello, Rust!");

    0
}
