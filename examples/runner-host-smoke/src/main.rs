use anyhow::{Context, Result, bail};
use greentic_interfaces::runner_host_v1;
use greentic_interfaces_wasmtime::component_v1_0;
use serde_json::Value;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec::Vec;
use wasmtime::component::{Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView, p2};

fn main() -> Result<()> {
    let component_path = ensure_component_artifact()?;
    let bytes = std::fs::read(&component_path)
        .with_context(|| format!("reading {}", component_path.display()))?;

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let component = component_v1_0::Component::instantiate(&engine, &bytes)?;
    let mut linker = Linker::new(&engine);

    p2::add_to_linker_sync(&mut linker)?;
    runner_host_v1::add_to_linker(&mut linker, |state: &mut AppState| &mut state.host)?;

    let mut store = Store::new(&engine, AppState::new()?);
    let instance = linker.instantiate(&mut store, &component)?;
    let iface_idx = instance
        .get_export_index(&mut store, None, "greentic:component/describe-v1@1.0.0")
        .context("describe-v1 interface not exported")?;

    let func_idx = instance
        .get_export_index(&mut store, Some(&iface_idx), "describe-json")
        .context("describe-v1 interface not exported")?;

    let describe = instance
        .get_typed_func::<(), (String,)>(&mut store, func_idx)
        .context("describe-json export not found")?;
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

    fn kv_get(&mut self, ns: String, key: String) -> wasmtime::Result<Option<String>> {
        Ok(Some(format!("{ns}:{key}")))
    }

    fn kv_put(&mut self, _ns: String, _key: String, _val: String) -> wasmtime::Result<()> {
        Ok(())
    }
}

struct AppState {
    wasi: WasiCtx,
    table: ResourceTable,
    host: HostMocks,
}

impl AppState {
    fn new() -> Result<Self> {
        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();
        Ok(Self {
            wasi,
            table: ResourceTable::new(),
            host: HostMocks,
        })
    }
}

impl WasiView for AppState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
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
    let target = workspace_root.join("target/wasm32-wasip2/debug/component_describe.wasm");

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
            "wasm32-wasip2",
        ])
        .env("CARGO_TARGET_DIR", workspace_root.join("target"))
        .status()
        .context("building component-describe example")?;

    if !status.success() {
        bail!("cargo build failed for component-describe");
    }

    Ok(())
}
