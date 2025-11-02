//! Wasmtime bindings for `greentic:host-import@0.4.0`.
#![allow(missing_docs)]

use wasmtime::component::{HasSelf, Linker};
use wasmtime::Result;

mod bindings {
    wasmtime::component::bindgen!({
        path: "wit-compat/greentic-host-import-0.4.0",
        world: "greentic:host-import/host-imports@0.4.0",
    });
}

use bindings::greentic::host_import::{http, secrets, telemetry};
use bindings::greentic::types_core::types;
pub use bindings::*;

/// Trait implemented by hosts to service the host-import surface.
pub trait HostImports: types::Host + secrets::Host + telemetry::Host + http::Host {}

impl<T> HostImports for T where T: types::Host + secrets::Host + telemetry::Host + http::Host {}

/// Registers all host imports with the provided linker.
pub fn add_to_linker<T>(linker: &mut Linker<T>) -> Result<()>
where
    T: HostImports + Send + Sync + 'static,
{
    bindings::HostImports::add_to_linker::<T, HasSelf<T>>(linker, |store: &mut T| store)
}

/// Returns the canonical package name for compatibility checks.
pub const PACKAGE_ID: &str = "greentic:host-import@0.4.0";
