#![deny(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]
#![doc = include_str!("../README.md")]

//! ABI-oriented bindings for Greentic WIT packages.

pub mod bindings;
#[cfg(not(target_arch = "wasm32"))]
pub mod wit_all;
#[cfg(not(target_arch = "wasm32"))]
pub use wit_all::*;
#[cfg(feature = "bindings-rust")]
pub mod mappers;
#[cfg(feature = "bindings-rust")]
pub mod validate;
