use greentic_interfaces_guest::component_v0_6::{
    component_descriptor, component_i18n, component_qa, component_runtime, component_schema,
};

struct Impl;

impl component_descriptor::Guest for Impl {
    fn get_component_info() -> Vec<u8> {
        vec![]
    }

    fn describe() -> Vec<u8> {
        vec![]
    }
}

impl component_schema::Guest for Impl {
    fn input_schema() -> Vec<u8> {
        vec![]
    }

    fn output_schema() -> Vec<u8> {
        vec![]
    }

    fn config_schema() -> Vec<u8> {
        vec![]
    }
}

impl component_runtime::Guest for Impl {
    fn run(_input: Vec<u8>, _state: Vec<u8>) -> component_runtime::RunResult {
        component_runtime::RunResult {
            output: vec![],
            new_state: vec![],
        }
    }
}

impl component_qa::Guest for Impl {
    fn qa_spec(_mode: component_qa::QaMode) -> Vec<u8> {
        vec![]
    }

    fn apply_answers(
        _mode: component_qa::QaMode,
        _current_config: Vec<u8>,
        _answers: Vec<u8>,
    ) -> Vec<u8> {
        vec![]
    }
}

impl component_i18n::Guest for Impl {
    fn i18n_keys() -> Vec<String> {
        vec![]
    }
}

greentic_interfaces_guest::export_component_v060!(Impl);

fn main() {}
