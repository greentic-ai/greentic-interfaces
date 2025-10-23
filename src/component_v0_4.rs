//! Typed bindings for `greentic:component@0.4.0`.

use wasmtime::Result;
use wasmtime::component::Linker;

mod bindings {
    wasmtime::component::bindgen!({
        path: "wit/greentic-component@0.4.0.wit",
        world: "component",
        async: true,
    });
}

pub use bindings::*;

/// Host-side trait that must be implemented to service the `control` interface.
pub use greentic::component::control::Host as ControlHost;

/// Registers the Greentic control interface with the provided linker.
pub fn add_control_to_linker<T, H>(
    linker: &mut Linker<T>,
    get_host: impl Fn(&mut T) -> &mut H + Send + Sync + Copy + 'static,
) -> Result<()>
where
    T: Send,
    H: ControlHost + Send,
{
    Component::add_to_linker(linker, get_host)
}

/// Returns the canonical package name for compatibility checks.
pub const PACKAGE_ID: &str = "greentic:component@0.4.0";
