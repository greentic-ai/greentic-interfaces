use crate::runtime_config_reader_v1_0::greentic::runtime_config::runtime_config as bindings;

/// Host trait for `greentic:runtime-config@1.0.0`.
pub use bindings::ConfigError;
pub use bindings::Host as RuntimeConfigHost;

/// Register the runtime-config world on the provided linker without exposing
/// generated module paths.
pub fn add_runtime_config_to_linker<T>(
    linker: &mut wasmtime::component::Linker<T>,
    get: fn(&mut T) -> &mut dyn RuntimeConfigHost,
) -> wasmtime::Result<()> {
    let mut instance = linker.instance("greentic:runtime-config/runtime-config@1.0.0")?;
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
