#![forbid(unsafe_code)]
#![allow(unexpected_cfgs)]

#[cfg(wasmtime_use_custom_tls)]
compile_error!("Do not build greentic-interfaces-wasmtime with `wasmtime_use_custom_tls`. Remove that cfg from RUSTFLAGS/.cargo/config.toml/CI.");

pub mod engine;
pub mod errors;
pub mod linker;

pub use engine::{build_engine, EngineOptions};
pub use errors::IfaceWasmError;
pub use linker::{
    add_clock_to_linker, add_fs_to_linker, add_secrets_to_linker, add_telemetry_to_linker,
    HostRegistrar, LinkerBuilder,
};
