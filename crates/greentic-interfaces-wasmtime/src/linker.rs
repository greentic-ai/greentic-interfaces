use wasmtime::component::Linker;

use crate::errors::IfaceWasmError;

/// Builder that wires common configuration before handing the linker back to callers.
pub struct LinkerBuilder<'a> {
    pub linker: Linker<()>,
    pub engine: &'a wasmtime::Engine,
}

impl<'a> LinkerBuilder<'a> {
    pub fn new(engine: &'a wasmtime::Engine) -> Self {
        Self {
            linker: Linker::new(engine),
            engine,
        }
    }

    pub fn finish(self) -> Linker<()> {
        self.linker
    }
}

pub struct HostRegistrar;

impl HostRegistrar {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HostRegistrar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "host-secrets")]
pub fn add_secrets_to_linker(linker: &mut Linker<()>) -> Result<(), IfaceWasmError> {
    let _ = linker;
    Ok(())
}

#[cfg(not(feature = "host-secrets"))]
pub fn add_secrets_to_linker(_linker: &mut Linker<()>) -> Result<(), IfaceWasmError> {
    Ok(())
}

#[cfg(feature = "host-telemetry")]
pub fn add_telemetry_to_linker(linker: &mut Linker<()>) -> Result<(), IfaceWasmError> {
    let _ = linker;
    Ok(())
}

#[cfg(not(feature = "host-telemetry"))]
pub fn add_telemetry_to_linker(_linker: &mut Linker<()>) -> Result<(), IfaceWasmError> {
    Ok(())
}

#[cfg(feature = "host-clock")]
pub fn add_clock_to_linker(linker: &mut Linker<()>) -> Result<(), IfaceWasmError> {
    let _ = linker;
    Ok(())
}

#[cfg(not(feature = "host-clock"))]
pub fn add_clock_to_linker(_linker: &mut Linker<()>) -> Result<(), IfaceWasmError> {
    Ok(())
}

#[cfg(feature = "host-fs")]
pub fn add_fs_to_linker(linker: &mut Linker<()>) -> Result<(), IfaceWasmError> {
    let _ = linker;
    Ok(())
}

#[cfg(not(feature = "host-fs"))]
pub fn add_fs_to_linker(_linker: &mut Linker<()>) -> Result<(), IfaceWasmError> {
    Ok(())
}
