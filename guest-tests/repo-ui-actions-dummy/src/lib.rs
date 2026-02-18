#[cfg(target_arch = "wasm32")]
wit_bindgen::generate!({
    path: "../../wit/greentic/repo-ui-actions@1.0.0/package.wit",
    world: "repo-ui-worker",
});

#[cfg(target_arch = "wasm32")]
struct RepoUiWorkerImpl;

#[cfg(target_arch = "wasm32")]
impl exports::greentic::repo_ui_actions::ui_action_api::Guest for RepoUiWorkerImpl {
    fn handle_action(
        tenant: String,
        page: String,
        action: String,
        input: exports::greentic::repo_ui_actions::ui_action_api::ActionInput,
    ) -> exports::greentic::repo_ui_actions::ui_action_api::ActionResult {
        let msg = format!(
            "tenant={tenant}, page={page}, action={action}, payload={}",
            input.payload
        );
        exports::greentic::repo_ui_actions::ui_action_api::ActionResult {
            success: true,
            message: msg,
            payload: Some(input.payload),
        }
    }
}

#[cfg(target_arch = "wasm32")]
export!(RepoUiWorkerImpl);

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
fn unused_host_stub() {}
