//! Rust bindings for `greentic:pack-export@0.4.0` without Wasmtime glue.
#![allow(missing_docs)]

#[cfg(feature = "bindings-rust")]
mod generated {
    wit_bindgen::generate!({
        path: "wit-compat/greentic-pack-export-0.4.0",
        world: "greentic:pack-export/pack-exports@0.4.0",
        generate_all,
        generate_unused_types: true,
    });
}

#[cfg(feature = "bindings-rust")]
pub use generated::*;

#[cfg(not(feature = "bindings-rust"))]
compile_error!("pack_export_v0_4 requires the `bindings-rust` feature");

/// Returns the canonical package name for compatibility checks.
pub const PACKAGE_ID: &str = "greentic:pack-export@0.4.0";
