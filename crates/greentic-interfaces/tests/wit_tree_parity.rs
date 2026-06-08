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
fn all_wit_mirror_trees_are_byte_identical() {
    // Three mirror trees MUST stay byte-identical to the canonical workspace
    // `wit/` so workspace dev builds, crates.io consumers (crate-local
    // fallback), and guest-side bindings all resolve the same worlds.
    //
    // `bundled-wit/` is NOT a mirror — it's a flattened publish payload that
    // intentionally carries extra dependency packages and may differ in content
    // (e.g. component@0.6.0 omits qa/i18n worlds). Its package-set coverage is
    // verified by `wit_trees_carry_the_same_package_set`; byte-identity only
    // applies to the three true mirror trees.
    let workspace = crate_dir().join("../../wit");
    let workspace = workspace.canonicalize().expect("workspace wit/");
    let workspace_pkgs = collect_packages(&workspace);

    let mirrors: Vec<(String, BTreeMap<String, PathBuf>)> = vec![
        (
            "crate-local (wit/)".to_string(),
            collect_packages(
                &crate_dir()
                    .join("wit")
                    .canonicalize()
                    .expect("crate-local wit/"),
            ),
        ),
        (
            "guest (interfaces-guest/wit/)".to_string(),
            collect_packages(
                &crate_dir()
                    .join("../greentic-interfaces-guest/wit")
                    .canonicalize()
                    .expect("guest wit/"),
            ),
        ),
    ];

    let mut drift = Vec::new();
    for (pkg, ws_path) in &workspace_pkgs {
        let ws_bytes = fs::read(ws_path).expect("read workspace package.wit");
        for (label, mirror_pkgs) in &mirrors {
            match mirror_pkgs.get(pkg) {
                None => drift.push(format!("  missing in {label}: {pkg}")),
                Some(mirror_path) => {
                    let mirror_bytes = fs::read(mirror_path).expect("read mirror package.wit");
                    if ws_bytes != mirror_bytes {
                        drift.push(format!(
                            "  byte-different: {pkg}\n    workspace: {}\n    {label}: {}",
                            ws_path.display(),
                            mirror_path.display()
                        ));
                    }
                }
            }
        }
    }
    // Detect extras in true mirror trees that don't exist in the canonical tree.
    for (label, mirror_pkgs) in &mirrors {
        for pkg in mirror_pkgs.keys() {
            if !workspace_pkgs.contains_key(pkg) {
                drift.push(format!("  extra in {label} (no workspace source): {pkg}"));
            }
        }
    }
    assert!(
        drift.is_empty(),
        "WIT mirrors diverged from canonical workspace `wit/`:\n{}\n\n\
         Mirror trees (crate-local, guest) must be byte-identical to workspace \
         `wit/` for every package. Bundled-wit is checked for shared packages only.",
        drift.join("\n")
    );
}
