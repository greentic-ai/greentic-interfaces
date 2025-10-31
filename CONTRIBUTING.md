# Contributing

## Local dependency overrides

This workspace depends on `greentic-types` from crates.io. If you have both repositories checked out side-by-side and want Cargo to use your local copy, copy `.cargo/local-patch.example.toml` to `.cargo/config.toml` and adjust the relative path as needed. The template contains:

```toml
[patch.crates-io]
greentic-types = { path = "../greentic-types" }
```

This override is optional and only affects your local build. CI always resolves `greentic-types` from crates.io.
