#![deny(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]
#![doc = include_str!("../README.md")]

//! ABI-oriented bindings for Greentic WIT packages.

pub mod bindings;
#[cfg(feature = "host-import-compat")]
pub mod host_import_v0_2;
#[cfg(feature = "host-import-compat")]
pub mod host_import_v0_4;
pub mod mappers;
pub mod validate;
