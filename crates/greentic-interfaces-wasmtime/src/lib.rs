#![forbid(unsafe_code)]

pub mod engine;
pub mod errors;
pub mod linker;
pub mod mappers;

pub use engine::{build_engine, EngineOptions};
pub use errors::IfaceWasmError;
pub use linker::{
    add_clock_to_linker, add_fs_to_linker, add_secrets_to_linker, add_telemetry_to_linker,
    HostRegistrar, LinkerBuilder,
};
pub use mappers::*;
