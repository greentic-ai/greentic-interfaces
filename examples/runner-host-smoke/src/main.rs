use anyhow::{bail, Context, Result};
use greentic_interfaces::runner_host_v1;
use greentic_interfaces_wasmtime::component_v1_0;
use serde_json::Value;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec::Vec;
use wasmtime::component::Linker;
use wasmtime::{Config, Engine, Store};

fn main() -> Result<()> {
    let component_path = ensure_component_artifact()?;
    let bytes = std::fs::read(&component_path)
        .with_context(|| format!("reading {}", component_path.display()))?;

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let component = component_v1_0::Component::instantiate(&engine, &bytes)?;
    let mut linker = Linker::new(&engine);

    runner_host_v1::add_to_linker(&mut linker, |state: &mut HostMocks| state)?;

    let mut store = Store::new(&engine, HostMocks::default());
    let instance = linker.instantiate(&mut store, &component)?;
    let describe = instance.get_typed_func::<(), (String,)>(&mut store, "describe-v1#describe-json")?;
    let (payload,) = describe.call(&mut store, ())?;

    // Ensure the JSON is at least well-formed.
    let json: Value = serde_json::from_str(&payload)?;
    println!("describe-json => {}", json);

    Ok(())
}

#[derive(Default)]
struct HostMocks;

impl runner_host_v1::RunnerHost for HostMocks {
    fn http_request(
        &mut self,
        _method: String,
        _url: String,
        _headers: Vec<String>,
        _body: Option<Vec<u8>>,
    ) -> wasmtime::Result<Result<Vec<u8>, String>> {
        Ok(Ok(Vec::new()))
    }

    fn secret_get(&mut self, name: String) -> wasmtime::Result<Result<String, String>> {
        Ok(Ok(format!("secret:{name}")))
    }

    fn kv_get(&mut self, ns: String, key: String) -> wasmtime::Result<Option<String>> {
        Ok(Some(format!("{ns}:{key}")))
    }

    fn kv_put(&mut self, _ns: String, _key: String, _val: String) -> wasmtime::Result<()> {
        Ok(())
    }
}

fn ensure_component_artifact() -> Result<PathBuf> {
    if let Ok(path) = env::var("COMPONENT_DESCRIBE_WASM") {
        return Ok(PathBuf::from(path));
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| anyhow::anyhow!("failed to determine workspace root"))?;
    let target = workspace_root.join("target/wasm32-unknown-unknown/debug/component_describe.wasm");

    if !target.exists() {
        build_component(&workspace_root)?;
    }

    if !target.exists() {
        bail!("component artifact missing at {}", target.display());
    }

    Ok(target)
}

fn build_component(workspace_root: &Path) -> Result<()> {
    let status = Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .current_dir(workspace_root)
        .args([
            "build",
            "--manifest-path",
            "examples/component-describe/Cargo.toml",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()
        .context("building component-describe example")?;

    if !status.success() {
        bail!("cargo build failed for component-describe");
    }

    Ok(())
}
