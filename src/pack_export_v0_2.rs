//! Typed bindings for `greentic:pack-export@0.2.0`.

mod bindings {
    wasmtime::component::bindgen!({
        path: "wit/greentic-pack-export@0.2.0.wit",
        world: "pack-exports",
    });
}

pub use bindings::*;

/// Returns the canonical package name for compatibility checks.
pub const PACKAGE_ID: &str = "greentic:pack-export@0.2.0";
