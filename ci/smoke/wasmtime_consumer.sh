#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMPDIR="$(mktemp -d)"
trap 'rm -rf "${TMPDIR}"' EXIT
export CARGO_TARGET_DIR="${ROOT}/target"

consumer_dir="${TMPDIR}/wasmtime-consumer"

cargo init --lib --name wasmtime_consumer "${consumer_dir}" >/dev/null

cat > "${consumer_dir}/Cargo.toml" <<TOML
[package]
name = "wasmtime_consumer"
version = "0.1.0"
edition = "2024"

[dependencies]
greentic-interfaces-wasmtime = { path = "${ROOT}/crates/greentic-interfaces-wasmtime" }
TOML

cat > "${consumer_dir}/src/lib.rs" <<'RS'
pub fn marker() -> &'static str {
    let _ = std::mem::size_of::<greentic_interfaces_wasmtime::component_v1_0::Component>();
    "ok"
}
RS

cargo build --manifest-path "${consumer_dir}/Cargo.toml"

cargo package --manifest-path "${ROOT}/crates/greentic-interfaces/Cargo.toml" --no-verify --allow-dirty >/dev/null
crate_file="$(ls -t "${ROOT}/target/package"/greentic-interfaces-*.crate | head -n1)"

if [[ -z "${crate_file}" ]]; then
    echo "No packaged greentic-interfaces crate artifact found under target/package"
    exit 1
fi

if ! tar -tf "${crate_file}" | grep -Eq '^greentic-interfaces-[^/]+/wit/'; then
    echo "Packaged greentic-interfaces crate is missing wit/** contents"
    exit 1
fi

echo "Smoke check passed: wasmtime consumer builds and packaged greentic-interfaces includes wit/**"
