//! Phase M1.4a: prove the `greentic:provider-instance-identity@0.1.0`
//! re-export from `greentic-interfaces-host` exposes the
//! `InstanceIdentity` world types for downstream host callers.

#![cfg(feature = "provider-instance-identity-v1")]

use greentic_interfaces_host::provider_instance_identity_v1::{
    InstanceIdentity, InstanceIdentityPre,
};

#[test]
fn instance_identity_host_reexport_compiles() {
    let pre_name = std::any::type_name::<InstanceIdentityPre<()>>();
    assert!(
        pre_name.contains("InstanceIdentityPre"),
        "InstanceIdentityPre re-export missing: {pre_name}"
    );

    let world_name = std::any::type_name::<InstanceIdentity>();
    assert!(
        world_name.contains("InstanceIdentity"),
        "InstanceIdentity re-export missing: {world_name}"
    );
}
