use crate::secrets_store_store_v1_0::greentic::secrets_store::secrets_store as bindings;

/// Host trait for `greentic:secrets-store@1.0.0`.
pub use bindings::Host as SecretsStoreHost;
pub use bindings::SecretsError;

/// Registers the secrets-store world on the provided linker without exposing
/// generated module paths.
pub fn add_secrets_store_to_linker<T>(
    linker: &mut wasmtime::component::Linker<T>,
    get: impl for<'a> Fn(&'a mut T) -> &'a mut (dyn SecretsStoreHost + 'a) + Send + Sync + 'static,
) -> anyhow::Result<()> {
    let mut instance = linker.instance("greentic:secrets-store/secrets-store@1.0.0")?;
    instance.func_wrap(
        "get",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (key,): (wasmtime::component::__internal::String,)| {
            let host = get(caller.data_mut());
            let result = host.get(key);
            Ok((result,))
        },
    )?;
    Ok(())
}
