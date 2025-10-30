use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let staged_root = Path::new("target").join("wit-bindgen");

    if staged_root.exists() {
        fs::remove_dir_all(&staged_root)?;
    }
    fs::create_dir_all(&staged_root)?;

    let wit_root = Path::new("wit");
    for entry in fs::read_dir(wit_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("wit") {
            stage_package(&path, &staged_root, wit_root)?;
        }
    }

    Ok(())
}

fn stage_package(src_path: &Path, staged_root: &Path, wit_root: &Path) -> Result<(), Box<dyn Error>> {
    let package_ref = read_package_ref(src_path)?;
    let dest_dir = staged_root.join(sanitize(&package_ref));
    fs::create_dir_all(&dest_dir)?;
    fs::copy(src_path, dest_dir.join("package.wit"))?;
    println!("cargo:rerun-if-changed={}", src_path.display());

    let mut visited = HashSet::new();
    stage_dependencies(&dest_dir, src_path, wit_root, &mut visited)?;
    Ok(())
}

fn stage_dependencies(
    parent_dir: &Path,
    source_path: &Path,
    wit_root: &Path,
    visited: &mut HashSet<String>,
) -> Result<(), Box<dyn Error>> {
    let deps = parse_deps(source_path)?;
    if deps.is_empty() {
        return Ok(());
    }

    let deps_dir = parent_dir.join("deps");
    fs::create_dir_all(&deps_dir)?;

    for dep in deps {
        if !visited.insert(dep.clone()) {
            continue;
        }

        let dep_src = wit_path(&dep, wit_root)?;
        let dep_dest = deps_dir.join(sanitize(&dep));
        fs::create_dir_all(&dep_dest)?;
        fs::copy(&dep_src, dep_dest.join("package.wit"))?;
        println!("cargo:rerun-if-changed={}", dep_src.display());

        stage_dependencies(&dep_dest, &dep_src, wit_root, visited)?;
    }

    Ok(())
}

fn wit_path(package_ref: &str, wit_root: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let (pkg, version) = package_ref
        .split_once('@')
        .ok_or_else(|| format!("invalid package reference: {package_ref}"))?;
    let file_name = format!("{}@{}.wit", pkg.replace(':', "-"), version);
    let path = wit_root.join(file_name);
    if !path.exists() {
        return Err(format!("missing WIT source for {package_ref}: {}", path.display()).into());
    }
    Ok(path)
}

fn read_package_ref(path: &Path) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("package ") {
            return Ok(rest.trim_end_matches(';').trim().to_string());
        }
    }
    Err(format!(
        "unable to locate package declaration in {}",
        path.display()
    )
    .into())
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

        let pkg = pkg_part.split('/').next().unwrap_or(pkg_part);
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

        let dep_ref = format!("{pkg}@{version}");
        if !deps.contains(&dep_ref) {
            deps.push(dep_ref);
        }
    }

    Ok(deps)
}

fn sanitize(package_ref: &str) -> String {
    package_ref.replace([':', '@'], "-")
}
