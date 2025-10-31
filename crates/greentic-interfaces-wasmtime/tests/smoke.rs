use greentic_interfaces_wasmtime::{build_engine, EngineOptions, LinkerBuilder};

#[tokio::test]
async fn engine_and_linker_build() {
    let engine = build_engine(EngineOptions::default()).expect("engine initialized");
    let builder = LinkerBuilder::new(&engine);
    let _linker = builder.finish();
}
