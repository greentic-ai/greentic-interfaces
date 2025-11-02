//! Rust bindings for `greentic:component@0.4.0` without Wasmtime glue.
#![allow(missing_docs)]

#[cfg(feature = "bindings-rust")]
mod generated {
    wit_bindgen::generate!({
        path: "wit-compat/greentic-component-0.4.0",
        world: "greentic:component/component@0.4.0",
        generate_all,
        generate_unused_types: true,
    });
}

#[cfg(feature = "bindings-rust")]
pub use generated::*;

#[cfg(not(feature = "bindings-rust"))]
compile_error!("component_v0_4 requires the `bindings-rust` feature");

/// Returns the canonical package name for compatibility checks.
pub const PACKAGE_ID: &str = "greentic:component@0.4.0";
