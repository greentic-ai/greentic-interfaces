//! Phase M1.4a: prove the `greentic:provider-instance-identity@0.1.0`
//! wasmtime bindings compile.
//!
//! A full end-to-end smoke test (load a fixture component and call
//! `identify_instance`) lands with the first provider implementation
//! in a follow-up; this file only forces the generated `InstanceIdentity`
//! and `InstanceIdentityPre` types to resolve.

use greentic_interfaces_wasmtime::instance_identity_v0_1::{InstanceIdentity, InstanceIdentityPre};

#[test]
fn instance_identity_bindings_compile() {
    let _ = core::mem::size_of::<InstanceIdentityPre<()>>();
    let _ = core::mem::size_of::<InstanceIdentity>();
}
