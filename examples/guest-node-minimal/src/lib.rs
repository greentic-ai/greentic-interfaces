#![deny(unsafe_code)]

#[cfg(target_arch = "wasm32")]
mod guest {
    use greentic_interfaces_guest::component::node::{
        ExecCtx, Guest, InvokeResult, LifecycleStatus, StreamEvent,
    };

    struct Component;

    impl Guest for Component {
        fn get_manifest() -> String {
            "{}".to_string()
        }

        fn on_start(_ctx: ExecCtx) -> Result<LifecycleStatus, String> {
            Ok(LifecycleStatus::Ok)
        }

        fn on_stop(_ctx: ExecCtx, _reason: String) -> Result<LifecycleStatus, String> {
            Ok(LifecycleStatus::Ok)
        }

        fn invoke(_ctx: ExecCtx, _op: String, _input: String) -> InvokeResult {
            InvokeResult::Ok("{}".to_string())
        }

        fn invoke_stream(_ctx: ExecCtx, _op: String, _input: String) -> Vec<StreamEvent> {
            vec![StreamEvent::Done]
        }
    }

    greentic_interfaces_guest::export_component_node!(Component);
}
