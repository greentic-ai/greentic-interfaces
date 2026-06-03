//! Phase D: prove the `greentic:provider-instance-identity/instance-identity-describe@0.1.0`
//! re-export from `greentic-interfaces-host` exposes the
//! `InstanceIdentityDescribe` world types for downstream host callers.

#![cfg(feature = "provider-instance-identity-describe-v1")]

use greentic_interfaces_host::provider_instance_identity_describe_v1::{
    InstanceIdentityDescribe, InstanceIdentityDescribePre,
};

#[test]
fn instance_identity_describe_host_reexport_compiles() {
    let _ = core::mem::size_of::<InstanceIdentityDescribePre<()>>();
    let _ = core::mem::size_of::<InstanceIdentityDescribe>();
}
