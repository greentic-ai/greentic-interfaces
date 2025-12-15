#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
use exports::greentic::distributor_api::distributor;

#[cfg(target_arch = "wasm32")]
wit_bindgen::generate!({
    path: "wit/greentic/distributor@1.0.0",
    world: "distributor-api",
    with: { "greentic:secrets-types/types@1.0.0": generate },
});

#[cfg(target_arch = "wasm32")]
struct DummyDistributor;

#[cfg(target_arch = "wasm32")]
impl distributor::Guest for DummyDistributor {
    fn resolve_component(
        _req: distributor::ResolveComponentRequest,
    ) -> distributor::ResolveComponentResponse {
        distributor::ResolveComponentResponse {
            component_status: distributor::ComponentStatus::Ready,
            digest: "sha256:dummydigest".to_string(),
            artifact_location: distributor::ArtifactLocation {
                kind: "file".to_string(),
                value: "/tmp/dummy.component.wasm".to_string(),
            },
            signature_summary: distributor::SignatureSummary {
                verified: false,
                signer: "n/a".to_string(),
                extra: "{}".to_string(),
            },
            cache_info: distributor::CacheInfo {
                size_bytes: 0,
                last_used_utc: "1970-01-01T00:00:00Z".to_string(),
                last_refreshed_utc: "1970-01-01T00:00:00Z".to_string(),
            },
            secret_requirements: Vec::new(),
        }
    }

    fn get_pack_status(_tenant_id: String, _environment_id: String, _pack_id: String) -> String {
        "\"ok\"".to_string()
    }

    fn get_pack_status_v2(
        _tenant_id: String,
        _environment_id: String,
        _pack_id: String,
    ) -> distributor::PackStatusResponse {
        distributor::PackStatusResponse {
            status: "ok".to_string(),
            secret_requirements: Vec::new(),
            extra: "{}".to_string(),
        }
    }

    fn warm_pack(_tenant_id: String, _environment_id: String, _pack_id: String) {}
}

#[cfg(target_arch = "wasm32")]
export!(DummyDistributor);
