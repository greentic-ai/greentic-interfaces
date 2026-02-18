#![allow(unsafe_code)]

#[cfg(target_arch = "wasm32")]
use exports::greentic::provider_schema_core::schema_core_api;

#[cfg(target_arch = "wasm32")]
wit_bindgen::generate!({
    path: "../../wit/greentic/provider/schema-core@1.0.0",
    world: "schema-core",
});

#[cfg(target_arch = "wasm32")]
struct ProviderCoreDummy;

#[cfg(target_arch = "wasm32")]
impl schema_core_api::Guest for ProviderCoreDummy {
    fn describe() -> Vec<u8> {
        br#"{"provider_type":"example.dummy","capabilities":["echo"],"ops":["echo"]}"#.to_vec()
    }

    fn validate_config(_config_json: Vec<u8>) -> Vec<u8> {
        br#"{"valid":true}"#.to_vec()
    }

    fn healthcheck() -> Vec<u8> {
        br#"{"status":"ok"}"#.to_vec()
    }

    fn invoke(_op: String, input_json: Vec<u8>) -> Vec<u8> {
        input_json
    }
}

#[cfg(target_arch = "wasm32")]
export!(ProviderCoreDummy);
