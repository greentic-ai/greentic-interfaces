use crate::http_client_client_v1_0::greentic::http::http_client as bindings;

/// Host trait for `greentic:http/client@1.0.0`.
pub use bindings::Host as HttpClientHost;
pub use bindings::{HostError as HttpClientError, Request, Response, TenantCtx};

/// Register the HTTP client world on the provided linker.
pub fn add_http_client_to_linker<T>(
    linker: &mut wasmtime::component::Linker<T>,
    get: fn(&mut T) -> &mut dyn HttpClientHost,
) -> anyhow::Result<()> {
    let mut instance = linker.instance("greentic:http/http-client@1.0.0")?;
    instance.func_wrap(
        "send",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (req, ctx): (bindings::Request, Option<bindings::TenantCtx>)| {
            let host = get(caller.data_mut());
            let result = host.send(req, ctx);
            Ok((result,))
        },
    )?;
    Ok(())
}
