use thiserror::Error;

#[derive(Debug, Error)]
pub enum IfaceWasmError {
    #[error("engine init: {0}")]
    Engine(String),
    #[error("linker: {0}")]
    Linker(String),
    #[error("instantiate: {0}")]
    Instantiate(String),
    #[error("call failed: {0}")]
    Call(String),
    #[error("type mapping: {0}")]
    Mapping(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
