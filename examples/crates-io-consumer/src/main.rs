fn main() {
    let allow_list = greentic_interfaces::bindings::greentic::interfaces_types::types::AllowList {
        domains: Vec::new(),
        ports: Vec::new(),
        protocols: Vec::new(),
    };

    println!(
        "greentic-interfaces bindings loaded; allow list has {} domains",
        allow_list.domains.len()
    );
}
