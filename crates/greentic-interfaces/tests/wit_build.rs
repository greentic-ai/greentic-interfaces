use std::path::Path;

use wit_parser::Resolve;

#[test]
fn staged_wit_packages_are_valid() {
    let staged_root = Path::new(env!("WIT_STAGING_DIR"));
    let entries = std::fs::read_dir(staged_root)
        .unwrap_or_else(|_| panic!("missing staged WIT packages in {}", staged_root.display()));

    for entry in entries {
        let entry = entry.expect("read staged entry");
        if !entry.path().is_dir() {
            continue;
        }
        let mut resolve = Resolve::new();
        resolve
            .push_dir(entry.path())
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", entry.path().display()));
    }
}

#[test]
fn oauth_broker_worlds_include_client() {
    use std::collections::BTreeSet;

    let staged_root = Path::new(env!("WIT_STAGING_DIR"));
    let package_dir = staged_root.join("greentic-oauth-broker-1.0.0");

    assert!(
        package_dir.exists(),
        "staged oauth-broker package missing at {}",
        package_dir.display()
    );

    let mut resolve = Resolve::new();
    let (pkg, _) = resolve
        .push_dir(&package_dir)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", package_dir.display()));

    let worlds: BTreeSet<String> = resolve.packages[pkg]
        .worlds
        .keys()
        .map(|name| name.to_string())
        .collect();

    assert!(
        worlds.contains("broker"),
        "expected existing broker world to remain"
    );
    assert!(
        worlds.contains("broker-client"),
        "expected additive broker-client world to be staged"
    );
}

#[test]
fn component_v0_v6_exports_node_interface() {
    use std::collections::BTreeSet;
    use wit_parser::WorldKey;

    let staged_root = Path::new(env!("WIT_STAGING_DIR"));
    let package_dir = staged_root.join("greentic-component-0.6.0");

    assert!(
        package_dir.exists(),
        "staged component package missing at {}",
        package_dir.display()
    );

    let mut resolve = Resolve::new();
    let (pkg, _) = resolve
        .push_dir(&package_dir)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", package_dir.display()));

    let world_id = resolve.packages[pkg]
        .worlds
        .get("component")
        .copied()
        .expect("missing component world");

    let world = &resolve.worlds[world_id];
    let export_names: BTreeSet<String> = world
        .exports
        .keys()
        .filter_map(|key| match key {
            WorldKey::Name(name) => Some(name.clone()),
            WorldKey::Interface(id) => resolve.interfaces[*id].name.clone(),
        })
        .collect();
    let import_names: BTreeSet<String> = world
        .imports
        .keys()
        .filter_map(|key| match key {
            WorldKey::Name(name) => Some(name.clone()),
            WorldKey::Interface(id) => resolve.interfaces[*id].name.clone(),
        })
        .collect();

    assert!(
        export_names.contains("node"),
        "expected component world to export node"
    );
    assert!(
        import_names.contains("control"),
        "expected component world to import control"
    );
}

#[test]
fn no_legacy_component_v0_v6_wit_mirrors_exist() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let guest =
        manifest_dir.join("../greentic-interfaces-guest/wit/greentic/component@0.6.0/package.wit");
    let wasmtime = manifest_dir
        .join("../greentic-interfaces-wasmtime/wit/greentic/component@0.6.0/package.wit");

    assert!(
        !guest.exists(),
        "legacy guest mirror should not exist at {}",
        guest.display()
    );
    assert!(
        !wasmtime.exists(),
        "legacy wasmtime mirror should not exist at {}",
        wasmtime.display()
    );
}
