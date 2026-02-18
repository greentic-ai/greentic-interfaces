pub fn canonical_wit_root() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"),
    );

    // Local crate checkout (if this crate carries its own WIT).
    let local = manifest_dir.join("wit");
    if local.exists() {
        return local
            .canonicalize()
            .expect("Failed to locate canonical WIT root");
    }

    // Workspace checkout from `crates/<this-crate>`.
    let workspace_sibling = manifest_dir.join("../greentic-interfaces/wit");
    if workspace_sibling.exists() {
        return workspace_sibling
            .canonicalize()
            .expect("Failed to locate canonical WIT root");
    }

    // `cargo package` verification from `target/package/<crate-version>`.
    let package_verify_workspace = manifest_dir.join("../../../crates/greentic-interfaces/wit");
    if package_verify_workspace.exists() {
        return package_verify_workspace
            .canonicalize()
            .expect("Failed to locate canonical WIT root");
    }

    // crates.io installs where sibling crates are unpacked as `greentic-interfaces-<ver>`.
    if let Some(parent) = manifest_dir.parent()
        && let Ok(entries) = std::fs::read_dir(parent)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !name.starts_with("greentic-interfaces-") {
                continue;
            }
            let candidate = path.join("wit");
            if candidate.exists() {
                return candidate
                    .canonicalize()
                    .expect("Failed to locate canonical WIT root");
            }
        }
    }

    panic!("Failed to locate canonical WIT root");
}
