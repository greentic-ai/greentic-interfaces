#![deny(unsafe_code)]
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

#[cfg(target_arch = "wasm32")]
mod guest {
    use greentic_interfaces_guest::component::node::{InvokeResult, NodeError};
    use greentic_interfaces_guest::component_entrypoint;

    fn describe_payload() -> String {
        r#"{"component":"guest-node-minimal"}"#.to_string()
    }

    fn handle_message(op: String, input: String) -> InvokeResult {
        match op.as_str() {
            "fail" => InvokeResult::Err(NodeError {
                code: "demo".to_string(),
                message: format!("error:{input}"),
                retryable: false,
                backoff_ms: None,
                details: None,
            }),
            _ => InvokeResult::Ok(format!("handled:{input}")),
        }
    }

    component_entrypoint!({
        manifest: describe_payload,
        invoke: handle_message,
    });
}
