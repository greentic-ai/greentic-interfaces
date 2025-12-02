#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
use exports::greentic::distributor_api::distributor_api;

#[cfg(target_arch = "wasm32")]
wit_bindgen::generate!({
    path: "../../crates/greentic-interfaces/wit/greentic/distributor@1.0.0",
    world: "distributor-api",
});

#[cfg(target_arch = "wasm32")]
struct DummyDistributor;

#[cfg(target_arch = "wasm32")]
impl distributor_api::Guest for DummyDistributor {
    fn resolve_component(
        _req: distributor_api::ResolveComponentRequest,
    ) -> distributor_api::ResolveComponentResponse {
        distributor_api::ResolveComponentResponse {
            component_status: distributor_api::ComponentStatus::Ready,
            digest: "sha256:dummydigest".to_string(),
            artifact_location: distributor_api::ArtifactLocation {
                kind: "file".to_string(),
                value: "/tmp/dummy.component.wasm".to_string(),
            },
            signature_summary: distributor_api::SignatureSummary {
                verified: false,
                signer: "n/a".to_string(),
                extra: "{}".to_string(),
            },
            cache_info: distributor_api::CacheInfo {
                size_bytes: 0,
                last_used_utc: "1970-01-01T00:00:00Z".to_string(),
                last_refreshed_utc: "1970-01-01T00:00:00Z".to_string(),
            },
        }
    }

    fn get_pack_status(_tenant_id: String, _environment_id: String, _pack_id: String) -> String {
        "\"ok\"".to_string()
    }

    fn warm_pack(_tenant_id: String, _environment_id: String, _pack_id: String) {}
}

#[cfg(target_arch = "wasm32")]
export!(DummyDistributor);
