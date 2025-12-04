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
