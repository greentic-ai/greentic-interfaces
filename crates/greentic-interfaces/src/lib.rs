#![deny(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]
#![doc = include_str!("../README.md")]

//! ABI-oriented bindings for Greentic WIT packages.

/// Returns the canonical WIT root shipped with this crate.
#[must_use]
pub fn wit_root() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for candidate in [
        manifest_dir.join("../../wit"),
        manifest_dir.join("wit"),
        manifest_dir.join("../greentic-interfaces/wit"),
    ] {
        if has_wit_files(&candidate) {
            return candidate;
        }
    }
    manifest_dir.join("wit")
}

fn has_wit_files(root: &std::path::Path) -> bool {
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|s| s.to_str()) == Some("wit") {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempTree {
        root: PathBuf,
    }

    impl TempTree {
        fn new(name: &str) -> Self {
            let nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_nanos(),
                Err(err) => panic!("system time before epoch: {err}"),
            };
            let root = std::env::temp_dir().join(format!("{name}-{}-{nanos}", std::process::id()));
            if let Err(err) = fs::create_dir_all(&root) {
                panic!("create temp tree: {err}");
            }
            Self { root }
        }
    }

    impl Drop for TempTree {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn has_wit_files_finds_nested_wit_package() {
        let temp = TempTree::new("greentic-wit-root");
        let nested = temp.root.join("greentic/example");
        if let Err(err) = fs::create_dir_all(&nested) {
            panic!("create nested WIT dir: {err}");
        }
        if let Err(err) = fs::write(nested.join("package.wit"), "package greentic:example;") {
            panic!("write WIT file: {err}");
        }

        assert!(has_wit_files(&temp.root));
    }

    #[test]
    fn has_wit_files_ignores_missing_or_non_wit_trees() {
        let temp = TempTree::new("greentic-empty-wit-root");
        if let Err(err) = fs::write(temp.root.join("README.md"), "not WIT") {
            panic!("write marker: {err}");
        }

        assert!(!has_wit_files(&temp.root));
        assert!(!has_wit_files(&temp.root.join("missing")));
    }

    #[test]
    fn wit_root_points_at_a_tree_with_wit_files() {
        let root = wit_root();
        assert!(
            has_wit_files(&root),
            "wit_root selected a path without WIT files: {}",
            root.display()
        );
    }
}

/// Stable ABI facade modules (`canonical`, `v0_6_0`) over generated bindings.
#[cfg(all(feature = "bindings-rust", not(target_arch = "wasm32")))]
pub mod abi;
#[cfg(not(target_arch = "wasm32"))]
pub mod bindings;
#[cfg(not(target_arch = "wasm32"))]
pub mod wit_all;
#[cfg(all(
    feature = "bindings-rust",
    feature = "wit-v0_6_0",
    not(target_arch = "wasm32")
))]
pub use abi::canonical;
#[cfg(all(
    feature = "bindings-rust",
    feature = "wit-v0_6_0",
    not(target_arch = "wasm32")
))]
pub use abi::v0_6_0;
#[cfg(all(
    feature = "bindings-rust",
    feature = "provider-common",
    not(target_arch = "wasm32")
))]
pub use bindings::provider_common_0_0_2_common;
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_imports)]
pub use wit_all::*;
#[cfg(all(
    feature = "bindings-rust",
    feature = "wit-v0_6_0",
    not(target_arch = "wasm32")
))]
pub mod mappers;
#[cfg(all(
    feature = "bindings-rust",
    feature = "wit-v0_6_0",
    not(target_arch = "wasm32")
))]
pub mod validate;
