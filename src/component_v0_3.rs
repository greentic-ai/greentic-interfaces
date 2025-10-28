//! Typed bindings for `greentic:component@0.3.0`.

mod bindings {
    wasmtime::component::bindgen!({
        path: "wit/greentic-component@0.3.0.wit",
        world: "component",
    });
}

pub use bindings::*;

/// Returns the canonical package name for backwards compatibility checks.
pub const PACKAGE_ID: &str = "greentic:component@0.3.0";
