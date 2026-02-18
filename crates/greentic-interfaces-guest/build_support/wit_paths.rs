use std::fs;
use std::path::{Path, PathBuf};

pub fn canonical_wit_root() -> PathBuf {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));
    let package_version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();

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
    if let Some(found) = crates_io_sibling_wit_root(&manifest_dir, &package_version) {
        return found;
    }

    panic!("Failed to locate canonical WIT root");
}

pub(crate) fn crates_io_sibling_wit_root(
    manifest_dir: &Path,
    package_version: &str,
) -> Option<PathBuf> {
    let parent = manifest_dir.parent()?;
    let prefix = "greentic-interfaces-";
    let mut candidates = Vec::new();

    let entries = fs::read_dir(parent).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if !name.starts_with(prefix) {
            continue;
        }

        let candidate = path.join("wit");
        if !candidate.exists() {
            continue;
        }

        candidates.push(candidate);
    }

    choose_sibling_wit_root(candidates, prefix, package_version)
}

pub(crate) fn choose_sibling_wit_root(
    mut candidates: Vec<PathBuf>,
    prefix: &str,
    package_version: &str,
) -> Option<PathBuf> {
    let exact_name = format!("{prefix}{package_version}");

    for candidate in &candidates {
        let candidate_name = candidate
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());
        if candidate_name == Some(exact_name.as_str()) {
            return candidate.canonicalize().ok();
        }
    }

    candidates.sort();
    candidates.pop()?.canonicalize().ok()
}
