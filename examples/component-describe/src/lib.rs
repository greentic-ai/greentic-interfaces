#![no_std]

extern crate alloc;

use alloc::string::String;

wit_bindgen::generate!({
    path: "../../crates/greentic-interfaces/wit/greentic/component@1.0.0",
    world: "component",
});

struct DescribeComponent;

impl exports::greentic::component::describe_v1::DescribeV1 for DescribeComponent {
    fn describe_json() -> String {
        String::from(
            r#"{"name":"component-describe","versions":[{"version":"1.0.0","schema":"{}"}]}"#,
        )
    }
}

wit_bindgen::export!(DescribeComponent);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
