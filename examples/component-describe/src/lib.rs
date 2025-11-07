use exports::greentic::component::describe_v1;

wit_bindgen::generate!({
    path: "../../crates/greentic-interfaces/wit/greentic/component@1.0.0",
    world: "component",
});

struct DescribeComponent;

impl describe_v1::Guest for DescribeComponent {
    fn describe_json() -> std::string::String {
        r#"{"name":"component-describe","versions":[{"version":"1.0.0","schema":"{}"}]}"#
            .to_string()
    }
}

export!(DescribeComponent);
