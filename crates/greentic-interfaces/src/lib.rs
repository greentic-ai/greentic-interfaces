#![deny(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]
#![doc = include_str!("../README.md")]

//! ABI-oriented bindings for Greentic WIT packages.

/// Returns the canonical WIT root shipped with this crate.
#[must_use]
pub fn wit_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("wit")
}

#[cfg(not(target_arch = "wasm32"))]
pub mod bindings;
#[cfg(not(target_arch = "wasm32"))]
pub mod wit_all;
#[cfg(all(
    feature = "bindings-rust",
    feature = "provider-common",
    not(target_arch = "wasm32")
))]
pub use bindings::provider_common_0_0_2_common;
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
pub use wit_all::*;
#[cfg(all(feature = "bindings-rust", not(target_arch = "wasm32")))]
pub mod mappers;
#[cfg(all(feature = "bindings-rust", not(target_arch = "wasm32")))]
pub mod validate;
