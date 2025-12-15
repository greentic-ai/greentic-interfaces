use anyhow::Result;
use greentic_interfaces_wasmtime::host_helpers::SecretsError;
use greentic_interfaces_wasmtime::{SecretsStoreHost, add_secrets_store_to_linker};
use wasmtime::component::Linker;
use wasmtime::{Config, Engine};

struct DummySecrets;

impl SecretsStoreHost for DummySecrets {
    fn get(
        &mut self,
        _key: wasmtime::component::__internal::String,
    ) -> std::result::Result<Option<wasmtime::component::__internal::Vec<u8>>, SecretsError> {
        Ok(None)
    }
}

struct HostState {
    secrets: DummySecrets,
}

#[test]
fn secrets_store_helper_wires_linker() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker: Linker<HostState> = Linker::new(&engine);

    add_secrets_store_to_linker(&mut linker, |state: &mut HostState| &mut state.secrets)?;

    Ok(())
}
