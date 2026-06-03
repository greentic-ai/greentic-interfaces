//! Phase D: prove the `greentic:provider-instance-identity/instance-identity-describe@0.1.0`
//! wasmtime bindings compile.
//!
//! Mirrors `provider_instance_identity_bindings.rs` for the new describe
//! world.

use greentic_interfaces_wasmtime::instance_identity_instance_identity_describe_v0_1::{
    InstanceIdentityDescribe, InstanceIdentityDescribePre,
};

#[test]
fn instance_identity_describe_bindings_compile() {
    let _ = core::mem::size_of::<InstanceIdentityDescribePre<()>>();
    let _ = core::mem::size_of::<InstanceIdentityDescribe>();
}
