use camino::Utf8PathBuf;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{env, fs};
use walkdir::WalkDir;

fn world_names_from_str(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if let Some(rest) = trimmed.strip_prefix("world ") {
                let name = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or("world")
                    .trim_end_matches('{')
                    .to_string();
                return Some(name);
            }
            None
        })
        .collect()
}

fn module_name_from_dir_and_world(dir: &str, world: &str) -> String {
    let mut parts = dir.split('@');
    let raw_name = parts
        .next()
        .unwrap_or(dir)
        .replace(['-', '/', ':', '.'], "_");
    let version = parts.next().unwrap_or("0.0.0");
    let mut ver_parts = version.trim_start_matches('v').split('.');
    let major = ver_parts.next().unwrap_or("0");
    let minor = ver_parts.next().unwrap_or("0");
    let world_part = world.replace('-', "_");

    if world_part == raw_name {
        format!("{raw_name}_v{major}_{minor}")
    } else {
        format!("{raw_name}_{world_part}_v{major}_{minor}")
    }
}

fn main() {
    let out_dir = Utf8PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let wit_root = Utf8PathBuf::from("wit").join("greentic");

    let mut modules: Vec<TokenStream> = Vec::new();

    for entry in WalkDir::new(&wit_root) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        if !entry.file_type().is_file() || entry.file_name() != "package.wit" {
            continue;
        }
        let package_path =
            Utf8PathBuf::from_path_buf(entry.path().to_path_buf()).expect("non-utf8 path");
        if package_path.components().any(|c| c.as_str() == "deps") {
            continue;
        }

        let package_dir = package_path
            .parent()
            .expect("package.wit must have a parent directory");
        let rel_dir = package_dir
            .strip_prefix(&wit_root)
            .expect("package path should live under wit/greentic");
        let dirname = package_dir
            .file_name()
            .map(|n| n.to_string())
            .unwrap_or_default();

        let content =
            fs::read_to_string(&package_path).unwrap_or_else(|_| panic!("Reading {package_path}"));

        let package_line = content
            .lines()
            .find(|line| line.trim_start().starts_with("package "))
            .unwrap_or_else(|| panic!("package declaration not found in {package_path}"));
        let package_ref = package_line
            .trim_start()
            .trim_start_matches("package")
            .trim()
            .trim_end_matches(';')
            .trim();
        let (package_id, version) = package_ref
            .rsplit_once('@')
            .unwrap_or((package_ref, "0.0.0"));

        let world_wit = package_dir.join("world.wit");
        let mut world_names = world_names_from_str(&content);
        if world_wit.exists()
            && world_names.is_empty()
            && let Ok(extra) = fs::read_to_string(&world_wit)
        {
            world_names = world_names_from_str(&extra);
        }

        if world_names.is_empty() {
            continue;
        }

        world_names.sort();

        for world_name in world_names {
            let module_name = module_name_from_dir_and_world(&dirname, &world_name);
            let mod_ident = format_ident!("{}", module_name);
            let world_spec = format!("{package_id}/{world_name}@{version}");
            let package_rel_path = format!("wit/greentic/{rel_dir}");

            let has_control_helpers = dirname.starts_with("component@")
                && content.contains("interface control")
                && content.contains("import control");

            let control_helpers = if has_control_helpers {
                quote! {
                    #[cfg(feature = "control-helpers")]
                    pub use bindings::greentic::component::control::Host as ControlHost;

                    #[cfg(feature = "control-helpers")]
                    pub use bindings::greentic::component::control::add_to_linker as add_control_to_linker;
                }
            } else {
                quote! {}
            };

            let module_tokens = quote! {
                pub mod #mod_ident {
                    mod bindings {
                        wasmtime::component::bindgen!({
                            path: #package_rel_path,
                            world: #world_spec
                        });
                    }

                    #[allow(unused_imports)]
                    pub use bindings::*;

                    /// Convenience shim to instantiate a component binary.
                    pub struct Component;
                    impl Component {
                        pub fn instantiate(
                            engine: &wasmtime::Engine,
                            component_wasm: &[u8],
                        ) -> anyhow::Result<wasmtime::component::Component> {
                            let component = wasmtime::component::Component::from_binary(engine, component_wasm)?;
                            Ok(component)
                        }
                    }

                    #control_helpers
                }
            };

            modules.push(module_tokens);
        }
    }

    modules.sort_by_key(|tokens| tokens.to_string());

    let src = quote! {
        // Auto-generated modules for each greentic WIT world discovered under `wit/greentic/*@*`.
        #(#modules)*
    };

    fs::create_dir_all(&out_dir).expect("create OUT_DIR");
    let gen_path = out_dir.join("gen_all_worlds.rs");
    fs::write(&gen_path, src.to_string()).expect("write generated bindings");

    println!("cargo:rerun-if-changed=wit");
}
