//! Phase M1.4a: prove the `greentic:provider-instance-identity@0.1.0`
//! re-export from `greentic-interfaces-host` exposes the
//! `InstanceIdentity` world types for downstream host callers.

#![cfg(feature = "provider-instance-identity-v1")]

use greentic_interfaces_host::provider_instance_identity_v1::{
    InstanceIdentity, InstanceIdentityPre,
};

#[test]
fn instance_identity_host_reexport_compiles() {
    let _ = core::mem::size_of::<InstanceIdentityPre<()>>();
    let _ = core::mem::size_of::<InstanceIdentity>();
}
