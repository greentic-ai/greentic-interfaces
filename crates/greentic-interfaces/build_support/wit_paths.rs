pub fn canonical_wit_root() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"),
    );

    let local = manifest_dir.join("wit");
    if local.exists() {
        return local
            .canonicalize()
            .expect("Failed to locate canonical WIT root");
    }

    manifest_dir
        .join("../greentic-interfaces/wit")
        .canonicalize()
        .expect("Failed to locate canonical WIT root")
}
