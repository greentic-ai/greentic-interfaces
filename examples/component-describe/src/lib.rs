#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
use exports::greentic::component::describe_v1;

#[cfg(target_arch = "wasm32")]
wit_bindgen::generate!({
    path: "../../crates/greentic-interfaces/wit/greentic/component@1.0.0",
    world: "component",
});

#[cfg(target_arch = "wasm32")]
struct DescribeComponent;

#[cfg(target_arch = "wasm32")]
impl describe_v1::Guest for DescribeComponent {
    fn describe_json() -> std::string::String {
        r#"{"name":"component-describe","versions":[{"version":"1.0.0","schema":"{}"}]}"#
            .to_string()
    }
}

#[cfg(target_arch = "wasm32")]
export!(DescribeComponent);
