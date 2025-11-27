wit_bindgen::generate!({
    path: "../../crates/greentic-interfaces/wit/greentic/repo-ui-actions@1.0.0/package.wit",
    world: "repo-ui-worker",
});

struct RepoUiWorkerImpl;

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

export!(RepoUiWorkerImpl);
