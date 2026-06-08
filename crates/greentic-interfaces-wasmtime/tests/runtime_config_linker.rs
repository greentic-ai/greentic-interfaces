use anyhow::Result;
use greentic_interfaces_wasmtime::{ConfigError, RuntimeConfigHost, add_runtime_config_to_linker};
use wasmtime::component::Linker;
use wasmtime::{Config, Engine};

fn wt<T>(result: wasmtime::Result<T>) -> Result<T> {
    result.map_err(|err| anyhow::anyhow!("{err}"))
}

struct DummyRuntimeConfig;

impl RuntimeConfigHost for DummyRuntimeConfig {
    fn get(
        &mut self,
        _key: wasmtime::component::__internal::String,
    ) -> std::result::Result<Option<wasmtime::component::__internal::Vec<u8>>, ConfigError> {
        Ok(None)
    }
}

struct HostState {
    runtime_config: DummyRuntimeConfig,
}

#[test]
fn runtime_config_helper_wires_linker() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = wt(Engine::new(&config))?;
    let mut linker: Linker<HostState> = Linker::new(&engine);

    wt(add_runtime_config_to_linker(
        &mut linker,
        |state: &mut HostState| &mut state.runtime_config,
    ))?;

    Ok(())
}
