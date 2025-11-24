use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use wit_bindgen_core::Files;
use wit_bindgen_core::wit_parser::Resolve;
use wit_bindgen_rust::Opts;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let staged_root = out_dir.join("wit-staging");
    reset_directory(&staged_root)?;

    let wit_root = Path::new("wit");
    let mut package_paths = Vec::new();
    discover_packages(wit_root, &mut package_paths)?;

    let mut staged = HashSet::new();
    for package_path in package_paths {
        let package_ref = read_package_ref(&package_path)?;
        if staged.insert(package_ref) {
            stage_package(&package_path, &staged_root, wit_root)?;
        }
    }

    let bindings_dir = generate_rust_bindings(&staged_root, &out_dir)?;

    println!("cargo:rustc-env=WIT_STAGING_DIR={}", staged_root.display());
    println!(
        "cargo:rustc-env=GREENTIC_INTERFACES_BINDINGS={}",
        bindings_dir.display()
    );

    Ok(())
}

fn stage_package(
    src_path: &Path,
    staged_root: &Path,
    wit_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let package_ref = read_package_ref(src_path)?;
    let dest_dir = staged_root.join(sanitize(&package_ref));
    fs::create_dir_all(&dest_dir)?;
    fs::copy(src_path, dest_dir.join("package.wit"))?;
    println!("cargo:rerun-if-changed={}", src_path.display());

    stage_dependencies(&dest_dir, src_path, wit_root)?;
    Ok(())
}

fn stage_dependencies(
    parent_dir: &Path,
    source_path: &Path,
    wit_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let deps = parse_deps(source_path)?;
    if deps.is_empty() {
        return Ok(());
    }
    if env::var("DEBUG_STAGE_DEPS").is_ok() {
        eprintln!(
            "[debug] staging deps for {} -> {:?}",
            source_path.display(),
            deps
        );
    }

    let deps_dir = parent_dir.join("deps");
    fs::create_dir_all(&deps_dir)?;

    for dep in deps {
        let dep_src = wit_path(&dep, wit_root)?;
        let dep_dest = deps_dir.join(sanitize(&dep));
        fs::create_dir_all(&dep_dest)?;
        fs::copy(&dep_src, dep_dest.join("package.wit"))?;
        if env::var("DEBUG_STAGE_DEPS").is_ok() {
            println!("cargo:warning=staging dependency {dep}");
        }
        println!("cargo:rerun-if-changed={}", dep_src.display());

        stage_dependencies(&dep_dest, &dep_src, wit_root)?;
    }

    Ok(())
}

fn wit_path(package_ref: &str, wit_root: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let (pkg, version) = package_ref
        .split_once('@')
        .ok_or_else(|| format!("invalid package reference: {package_ref}"))?;
    let base_pkg = pkg.split('/').next().unwrap_or(pkg);
    let target_root = format!("{base_pkg}@{version}");
    let mut fallback = None;
    if let Some(found) = find_package_recursive(wit_root, package_ref, &target_root, &mut fallback)?
    {
        return Ok(found);
    }
    if let Some(path) = fallback {
        return Ok(path);
    }
    Err(format!("missing WIT source for {package_ref}").into())
}

fn read_package_ref(path: &Path) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("package ") {
            return Ok(rest.trim_end_matches(';').trim().to_string());
        }
    }
    Err(format!("unable to locate package declaration in {}", path.display()).into())
}

fn parse_deps(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let mut deps = Vec::new();

    for line in contents.lines() {
        let trimmed = line.trim_start();
        let rest = if let Some(rest) = trimmed.strip_prefix("use ") {
            rest
        } else if let Some(rest) = trimmed.strip_prefix("import ") {
            rest
        } else {
            continue;
        };

        let token = rest.split_whitespace().next().unwrap_or("");
        let token = token.trim_end_matches(';');
        let token = token.split(".{").next().unwrap_or(token);
        let token = token.split('{').next().unwrap_or(token);

        let (pkg_part, version_part) = match token.split_once('@') {
            Some(parts) => parts,
            None => continue,
        };

        let base_pkg = pkg_part.split('/').next().unwrap_or(pkg_part);
        let mut version = String::new();
        for ch in version_part.chars() {
            if ch.is_ascii_alphanumeric() || ch == '.' || ch == '-' || ch == '_' {
                version.push(ch);
            } else {
                break;
            }
        }
        while version.ends_with('.') {
            version.pop();
        }
        if version.is_empty() {
            continue;
        }

        let dep_ref = format!("{base_pkg}@{version}");
        if !deps.contains(&dep_ref) {
            deps.push(dep_ref);
        }
    }

    Ok(deps)
}

fn sanitize(package_ref: &str) -> String {
    package_ref.replace([':', '@', '/'], "-")
}

fn generate_rust_bindings(staged_root: &Path, out_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let bindings_dir = out_dir.join("bindings");
    reset_directory(&bindings_dir)?;

    let mut package_paths = Vec::new();
    let mut inserted = HashSet::new();

    for entry in fs::read_dir(staged_root)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let package_path = path.join("package.wit");
        if !package_path.exists() {
            continue;
        }

        let package_ref = read_package_ref(&package_path)?;
        if !inserted.insert(package_ref) {
            continue;
        }

        package_paths.push(path);
    }

    if package_paths.is_empty() {
        return Err("no WIT worlds discovered to generate bindings for".into());
    }

    package_paths.sort();

    let opts = Opts {
        generate_all: true,
        generate_unused_types: true,
        ..Default::default()
    };

    let mut default_module = None;
    let mut mod_rs = String::new();

    for path in package_paths {
        let mut resolve = Resolve::new();
        let (pkg, _) = resolve.push_dir(&path)?;
        let package = &resolve.packages[pkg];

        let mut worlds: Vec<_> = package.worlds.iter().collect();
        worlds.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));

        for (world_name, world_id) in worlds {
            let module_name = module_name(&package.name, world_name);
            let mut files = Files::default();
            opts.clone()
                .build()
                .generate(&resolve, *world_id, &mut files)?;

            let mut combined = Vec::new();
            for (_, contents) in files.iter() {
                combined.extend_from_slice(contents);
            }
            fs::write(bindings_dir.join(format!("{module_name}.rs")), combined)?;
            mod_rs.push_str(&format!(
                "pub mod {module_name} {{ include!(concat!(env!(\"GREENTIC_INTERFACES_BINDINGS\"), \"/{module_name}.rs\")); }}\n"
            ));

            if package.name.namespace == "greentic"
                && package.name.name == "interfaces-pack"
                && matches!(&package.name.version, Some(ver) if ver.major == 0 && ver.minor == 1)
                && world_name == "component"
            {
                default_module = Some(module_name.clone());
            }
        }
    }

    if let Some(default) = default_module {
        mod_rs.push_str(&format!("pub use {default}::*;\n"));
    }

    fs::write(bindings_dir.join("mod.rs"), mod_rs)?;

    Ok(bindings_dir)
}

fn module_name(name: &wit_bindgen_core::wit_parser::PackageName, world: &str) -> String {
    let formatted = format!("{name}-{world}");
    sanitize(&formatted).replace(['-', '.'], "_")
}

fn reset_directory(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)?;
    Ok(())
}

fn discover_packages(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let package_file = path.join("package.wit");
            if package_file.exists() {
                out.push(package_file);
            }
            discover_packages(&path, out)?;
        } else if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("wit") {
            out.push(path);
        }
    }
    Ok(())
}

fn find_package_recursive(
    dir: &Path,
    package_ref: &str,
    target_root: &str,
    fallback: &mut Option<PathBuf>,
) -> Result<Option<PathBuf>, Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let package_file = path.join("package.wit");
            if package_file.exists() {
                let entry_package = read_package_ref(&package_file)?;
                if entry_package == package_ref {
                    return Ok(Some(package_file));
                }
                if fallback.is_none() && entry_package == target_root {
                    *fallback = Some(package_file.clone());
                }
            }
            if let Some(found) = find_package_recursive(&path, package_ref, target_root, fallback)?
            {
                return Ok(Some(found));
            }
        } else if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("wit") {
            let entry_package = read_package_ref(&path)?;
            if entry_package == package_ref {
                return Ok(Some(path));
            }
            if fallback.is_none() && entry_package == target_root {
                *fallback = Some(path.clone());
            }
        }
    }
    Ok(None)
}
