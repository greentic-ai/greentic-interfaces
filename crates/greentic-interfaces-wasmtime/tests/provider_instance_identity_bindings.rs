//! Phase M1.4a: prove the `greentic:provider-instance-identity@0.1.0`
//! wasmtime bindings compile and expose the expected entry points.
//!
//! A full end-to-end smoke test (load a fixture component and call
//! `identify_instance`) lands with the first provider implementation in
//! a follow-up; this file only asserts the binding shape today.

use greentic_interfaces_wasmtime::instance_identity_v0_1::{InstanceIdentity, InstanceIdentityPre};
use wasmtime::component::Linker;
use wasmtime::{Config, Engine};

#[test]
fn instance_identity_bindings_compile() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).expect("wasmtime engine");

    // Sanity-check the linker can be parametrised against the generated world.
    // We do not link or instantiate anything — providers that opt into the
    // contract will exercise the call path in their own integration tests.
    let _linker: Linker<()> = Linker::new(&engine);

    let pre_name = std::any::type_name::<InstanceIdentityPre<()>>();
    assert!(
        pre_name.contains("InstanceIdentityPre"),
        "InstanceIdentityPre type missing from generated bindings: {pre_name}"
    );

    let world_name = std::any::type_name::<InstanceIdentity>();
    assert!(
        world_name.contains("InstanceIdentity"),
        "InstanceIdentity type missing from generated bindings: {world_name}"
    );
}
