use wasmtime::Config;

use crate::errors::IfaceWasmError;

#[derive(Clone, Debug, Default)]
pub struct EngineOptions {
    pub fuel: bool,
    pub max_wasm_stack: Option<usize>,
}

pub fn build_engine(opts: EngineOptions) -> Result<wasmtime::Engine, IfaceWasmError> {
    let mut cfg = Config::new();
    cfg.wasm_component_model(true);

    #[cfg(feature = "stable-wasmtime")]
    {
        cfg.async_support(true);
    }

    #[cfg(feature = "nightly-wasmtime")]
    {
        cfg.async_support(true);
        // placeholder for nightly-specific knobs; update when we need fibers/etc.
    }

    if opts.fuel {
        cfg.consume_fuel(true);
    }

    if let Some(bytes) = opts.max_wasm_stack {
        cfg.max_wasm_stack(bytes);
    }

    wasmtime::Engine::new(&cfg).map_err(|e| IfaceWasmError::Engine(e.to_string()))
}
