//! Rust bindings for `greentic:types-core@0.4.0` without Wasmtime glue.
#![allow(missing_docs)]

#[cfg(feature = "bindings-rust")]
mod generated {
    wit_bindgen::generate!({
        path: "wit-compat/greentic-types-core-0.4.0",
        world: "greentic:types-core/core@0.4.0",
        generate_all,
        generate_unused_types: true,
    });
}

#[cfg(feature = "bindings-rust")]
pub use generated::*;

#[cfg(not(feature = "bindings-rust"))]
compile_error!("types_core_v0_4 requires the `bindings-rust` feature");

/// Returns the canonical package name for compatibility checks.
pub const PACKAGE_ID: &str = "greentic:types-core@0.4.0";
