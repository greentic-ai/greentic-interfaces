//! Phase M1.4a: catch the failure mode Codex flagged on PR #144.
//!
//! Four WIT trees ship in this repo today:
//!
//! - `wit/` at the workspace root (canonical authoring source)
//! - `crates/greentic-interfaces/wit/` (crate-local copy — what
//!   `wit_root()` falls back to after a crates.io publish, since the
//!   workspace-relative `../../wit/` no longer exists)
//! - `crates/greentic-interfaces/bundled-wit/<sanitized>/package.wit`
//!   (the publish payload that `greentic-interfaces/Cargo.toml`
//!   declares via `include = [..., "bundled-wit/**", ...]`)
//! - `crates/greentic-interfaces-guest/wit/` (guest staging tree)
//!
//! Adding a new package and forgetting one of the trees is a silent
//! post-publish failure: workspace builds stay green, but downstream
//! consumers can't discover the new world. This test fails immediately
//! when the sets diverge.

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

const WORKSPACE_RELATIVE_TREES: &[&str] = &["../../wit", "../greentic-interfaces-guest/wit"];
const CRATE_RELATIVE_TREES: &[&str] = &["wit"];
const BUNDLED_DIR: &str = "bundled-wit";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_package_ref(path: &Path) -> Option<String> {
    let contents = fs::read_to_string(path).ok()?;
    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("package ") {
            return Some(rest.trim_end_matches(';').trim().to_string());
        }
    }
    None
}

fn collect_packages(root: &Path) -> BTreeMap<String, PathBuf> {
    let mut packages = BTreeMap::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip nested deps/ folders so we only collect canonical
                // top-level packages, mirroring the build-script discovery.
                if path.file_name().and_then(|n| n.to_str()) == Some("deps") {
                    continue;
                }
                stack.push(path);
            } else if path.file_name() == Some(OsStr::new("package.wit"))
                && let Some(pkg) = read_package_ref(&path)
            {
                packages.entry(pkg).or_insert(path);
            }
        }
    }
    packages
}

fn collect_bundled_packages(root: &Path) -> BTreeMap<String, PathBuf> {
    let mut packages = BTreeMap::new();
    let Ok(entries) = fs::read_dir(root) else {
        panic!("missing bundled-wit root at {}", root.display());
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let package_file = path.join("package.wit");
        if let Some(pkg) = read_package_ref(&package_file) {
            packages.insert(pkg, package_file);
        }
    }
    packages
}

#[test]
fn wit_trees_carry_the_same_package_set() {
    let canonical_root = crate_dir().join(WORKSPACE_RELATIVE_TREES[0]);
    let canonical = collect_packages(&canonical_root);
    assert!(
        !canonical.is_empty(),
        "canonical wit/ root at {} is empty — repository layout drifted",
        canonical_root.display()
    );
    let canonical_pkgs: Vec<&String> = canonical.keys().collect();

    // Trees to verify against the canonical set — canonical itself is the
    // reference, so don't list it (would trivially compare to itself).
    let mut all_trees: Vec<(String, BTreeMap<String, PathBuf>)> = Vec::new();
    for relative in &WORKSPACE_RELATIVE_TREES[1..] {
        let root = crate_dir().join(relative);
        all_trees.push((relative.to_string(), collect_packages(&root)));
    }
    for relative in CRATE_RELATIVE_TREES {
        let root = crate_dir().join(relative);
        all_trees.push((relative.to_string(), collect_packages(&root)));
    }
    let bundled_root = crate_dir().join(BUNDLED_DIR);
    all_trees.push((
        BUNDLED_DIR.to_string(),
        collect_bundled_packages(&bundled_root),
    ));

    let mut missing: Vec<String> = Vec::new();
    for (label, tree) in &all_trees {
        for pkg in &canonical_pkgs {
            if !tree.contains_key(*pkg) {
                missing.push(format!("  {label}: missing {pkg}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "WIT trees diverged from canonical workspace `wit/`:\n{}\n\nA new package was added to wit/ but at least one of the publish/guest copies was missed. Mirror the package.wit into every tree above so crates.io consumers can discover the world.",
        missing.join("\n")
    );
}

#[test]
fn workspace_and_crate_local_wit_trees_are_byte_identical() {
    // `wit_root()` in src/lib.rs falls back to `manifest_dir.join("wit")`
    // when `../../wit` is unavailable (the crates.io install case).
    // The two trees MUST stay byte-identical so a workspace dev build and
    // a published consumer see exactly the same world set.
    let workspace = crate_dir().join("../../wit");
    let crate_local = crate_dir().join("wit");
    let workspace = workspace.canonicalize().expect("workspace wit/");
    let crate_local = crate_local.canonicalize().expect("crate-local wit/");

    let workspace_pkgs = collect_packages(&workspace);
    let crate_pkgs = collect_packages(&crate_local);

    let mut drift = Vec::new();
    for (pkg, ws_path) in &workspace_pkgs {
        match crate_pkgs.get(pkg) {
            None => drift.push(format!("  missing in crate-local: {pkg}")),
            Some(crate_path) => {
                let ws_bytes = fs::read(ws_path).expect("read workspace package.wit");
                let crate_bytes = fs::read(crate_path).expect("read crate-local package.wit");
                if ws_bytes != crate_bytes {
                    drift.push(format!(
                        "  byte-different: {pkg}\n    workspace: {}\n    crate-local: {}",
                        ws_path.display(),
                        crate_path.display()
                    ));
                }
            }
        }
    }
    for pkg in crate_pkgs.keys() {
        if !workspace_pkgs.contains_key(pkg) {
            drift.push(format!(
                "  extra in crate-local (no workspace source): {pkg}"
            ));
        }
    }
    assert!(
        drift.is_empty(),
        "workspace `wit/` and `crates/greentic-interfaces/wit/` are out of sync:\n{}",
        drift.join("\n")
    );
}
