use crate::messaging_session_session_v1_0::greentic::messaging::session_api as bindings;

/// Host trait for `greentic:messaging/session@1.0.0`.
pub use bindings::Host as MessagingSessionHost;
pub use bindings::{HostError as MessagingSessionError, OpAck, OutboundMessage, TenantCtx};

/// Register the messaging-session world on the provided linker.
pub fn add_messaging_session_to_linker<T>(
    linker: &mut wasmtime::component::Linker<T>,
    get: fn(&mut T) -> &mut dyn MessagingSessionHost,
) -> anyhow::Result<()> {
    let mut instance = linker.instance("greentic:messaging/session-api@1.0.0")?;
    instance.func_wrap(
        "send",
        move |mut caller: wasmtime::StoreContextMut<'_, T>,
              (message, ctx): (bindings::OutboundMessage, bindings::TenantCtx)| {
            let host = get(caller.data_mut());
            let result = host.send(message, ctx);
            Ok((result,))
        },
    )?;
    Ok(())
}
