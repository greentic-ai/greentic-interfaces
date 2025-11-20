//! Unified Wasmtime bindings for every WIT world shipped with this crate.
#![allow(clippy::all)]
#![allow(missing_docs)]

macro_rules! declare_world {
    (
        mod $mod_name:ident,
        path = $path_literal:literal,
        world = $world_literal:literal
        $(, legacy = { $($legacy:item)* } )?
    ) => {
        pub mod $mod_name {
            mod bindings {
                wasmtime::component::bindgen!({
                    path: $path_literal,
                    world: $world_literal,
                });
            }

            #[allow(unused_imports)]
            pub use bindings::*;

            $(
                $($legacy)*
            )?
        }
    };
}

#[cfg(feature = "describe-v1")]
declare_world!(
    mod component_describe_v1,
    path = "wit/greentic/component@1.0.0",
    world = "greentic:component/component@1.0.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:component@1.0.0";
    }
);

#[cfg(feature = "component-v0-4")]
declare_world!(
    mod component_v0_4,
    path = "wit/greentic/component@0.4.0",
    world = "greentic:component/component@0.4.0",
    legacy = {
        use anyhow::Result as AnyResult;
        use wasmtime::component::{Component as WasmtimeComponent, Linker};
        use wasmtime::StoreContextMut;

        pub use bindings::greentic::component::control::Host as ControlHost;

        /// Registers the Greentic control interface with the provided linker.
        pub fn add_control_to_linker<T>(
            linker: &mut Linker<T>,
            get_host: impl Fn(&mut T) -> &mut (dyn ControlHost + Send + Sync + 'static)
                + Send
                + Sync
                + Copy
                + 'static,
        ) -> wasmtime::Result<()>
        where
            T: Send + 'static,
        {
            let mut inst = linker.instance("greentic:component/control@0.4.0")?;

            inst.func_wrap(
                "should-cancel",
                move |mut caller: StoreContextMut<'_, T>, (): ()| {
                    let host = get_host(caller.data_mut());
                    let result = host.should_cancel();
                    Ok((result,))
                },
            )?;

            inst.func_wrap(
                "yield-now",
                move |mut caller: StoreContextMut<'_, T>, (): ()| {
                    let host = get_host(caller.data_mut());
                    host.yield_now();
                    Ok(())
                },
            )?;

            Ok(())
        }

        /// Back-compat shim for instantiating the component.
        pub struct Component;

        impl Component {
            /// Loads the component from raw bytes, mirroring the old helper.
            pub fn instantiate(
                engine: &wasmtime::Engine,
                component_wasm: &[u8],
            ) -> AnyResult<WasmtimeComponent> {
                Ok(WasmtimeComponent::from_binary(engine, component_wasm)?)
            }
        }

        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:component@0.4.0";
    }
);

#[cfg(feature = "pack-export-v0-4")]
declare_world!(
    mod pack_export_v0_4,
    path = "wit/greentic/pack-export@0.4.0",
    world = "greentic:pack-export/pack-exports@0.4.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:pack-export@0.4.0";
    }
);

#[cfg(feature = "types-core-v0-4")]
declare_world!(
    mod types_core_v0_4,
    path = "wit/greentic/types-core@0.4.0",
    world = "greentic:types-core/core@0.4.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:types-core@0.4.0";
    }
);

#[cfg(feature = "host-import-v0-6")]
declare_world!(
    mod host_import_v0_6,
    path = "wit/greentic/host-import@0.6.0",
    world = "greentic:host-import/host-imports@0.6.0",
    legacy = {
        use wasmtime::component::Linker;
        use wasmtime::{Result, StoreContextMut};

        pub use bindings::greentic::host_import::{http, mcp, secrets, session, state, telemetry};
        pub use bindings::greentic::interfaces_types::types as iface_types;
        pub use bindings::greentic::types_core::types;

        /// Trait implemented by hosts to service the component imports.
        pub trait HostImports {
            fn secrets_get(
                &mut self,
                key: String,
                ctx: Option<types::TenantCtx>,
            ) -> Result<Result<String, types::IfaceError>>;

            fn telemetry_emit(
                &mut self,
                span_json: String,
                ctx: Option<types::TenantCtx>,
            ) -> Result<()>;

            fn http_fetch(
                &mut self,
                req: http::HttpRequest,
                ctx: Option<types::TenantCtx>,
            ) -> Result<Result<http::HttpResponse, types::IfaceError>>;

            fn mcp_exec(
                &mut self,
                component: String,
                action: String,
                args_json: String,
                ctx: Option<types::TenantCtx>,
            ) -> Result<Result<String, types::IfaceError>>;

            fn state_get(
                &mut self,
                key: iface_types::StateKey,
                ctx: Option<types::TenantCtx>,
            ) -> Result<Result<String, types::IfaceError>>;

            fn state_set(
                &mut self,
                key: iface_types::StateKey,
                value_json: String,
                ctx: Option<types::TenantCtx>,
            ) -> Result<Result<state::OpAck, types::IfaceError>>;

            fn session_update(
                &mut self,
                cursor: iface_types::SessionCursor,
                ctx: Option<types::TenantCtx>,
            ) -> Result<Result<String, types::IfaceError>>;
        }

        /// Registers the host import functions with the provided linker.
        pub fn add_to_linker<T>(
            linker: &mut Linker<T>,
            get_host: impl Fn(&mut T) -> &mut (dyn HostImports + Send + Sync + 'static)
                + Send
                + Sync
                + Copy
                + 'static,
        ) -> Result<()>
        where
            T: Send + 'static,
        {
            let mut secrets = linker.instance("greentic:host-import/secrets@0.6.0")?;
            secrets.func_wrap(
                "get",
                move |mut caller: StoreContextMut<'_, T>, (key, ctx): (String, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.secrets_get(key, ctx).map(|res| (res,))
                },
            )?;

            let mut telemetry = linker.instance("greentic:host-import/telemetry@0.6.0")?;
            telemetry.func_wrap(
                "emit",
                move |mut caller: StoreContextMut<'_, T>, (span, ctx): (String, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.telemetry_emit(span, ctx)
                },
            )?;

            let mut http_iface = linker.instance("greentic:host-import/http@0.6.0")?;
            http_iface.func_wrap(
                "fetch",
                move |mut caller: StoreContextMut<'_, T>, (req, ctx): (http::HttpRequest, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.http_fetch(req, ctx).map(|res| (res,))
                },
            )?;

            let mut mcp_iface = linker.instance("greentic:host-import/mcp@0.6.0")?;
            mcp_iface.func_wrap(
                "exec",
                move |mut caller: StoreContextMut<'_, T>,
                      (component, action, args, ctx): (String, String, String, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.mcp_exec(component, action, args, ctx).map(|res| (res,))
                },
            )?;

            let mut state_iface = linker.instance("greentic:host-import/state@0.6.0")?;
            state_iface.func_wrap(
                "get",
                move |mut caller: StoreContextMut<'_, T>,
                      (key, ctx): (iface_types::StateKey, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.state_get(key, ctx).map(|res| (res,))
                },
            )?;
            state_iface.func_wrap(
                "set",
                move |mut caller: StoreContextMut<'_, T>,
                      (key, value, ctx): (iface_types::StateKey, String, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.state_set(key, value, ctx).map(|res| (res,))
                },
            )?;

            let mut session_iface = linker.instance("greentic:host-import/session@0.6.0")?;
            session_iface.func_wrap(
                "update",
                move |mut caller: StoreContextMut<'_, T>,
                      (cursor, ctx): (iface_types::SessionCursor, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.session_update(cursor, ctx).map(|res| (res,))
                },
            )?;

            Ok(())
        }

        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:host-import@0.6.0";
    }
);

#[cfg(feature = "host-import-v0-4")]
declare_world!(
    mod host_import_v0_4,
    path = "wit/greentic/host-import@0.4.0",
    world = "greentic:host-import/host-imports@0.4.0",
    legacy = {
        use wasmtime::component::Linker;
        use wasmtime::{Result, StoreContextMut};

        pub use bindings::greentic::host_import::{http, secrets, telemetry};
        pub use bindings::greentic::types_core::types;

        /// Trait implemented by hosts to service the component imports.
        pub trait HostImports {
            fn secrets_get(
                &mut self,
                key: String,
                ctx: Option<types::TenantCtx>,
            ) -> Result<Result<String, types::IfaceError>>;

            fn telemetry_emit(
                &mut self,
                span_json: String,
                ctx: Option<types::TenantCtx>,
            ) -> Result<()>;

            fn http_fetch(
                &mut self,
                req: http::HttpRequest,
                ctx: Option<types::TenantCtx>,
            ) -> Result<Result<http::HttpResponse, types::IfaceError>>;
        }

        /// Registers the host import functions with the provided linker.
        pub fn add_to_linker<T>(
            linker: &mut Linker<T>,
            get_host: impl Fn(&mut T) -> &mut (dyn HostImports + Send + Sync + 'static)
                + Send
                + Sync
                + Copy
                + 'static,
        ) -> Result<()>
        where
            T: Send + 'static,
        {
            let mut secrets = linker.instance("greentic:host-import/secrets@0.4.0")?;
            secrets.func_wrap(
                "get",
                move |mut caller: StoreContextMut<'_, T>, (key, ctx): (String, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.secrets_get(key, ctx).map(|res| (res,))
                },
            )?;

            let mut telemetry = linker.instance("greentic:host-import/telemetry@0.4.0")?;
            telemetry.func_wrap(
                "emit",
                move |mut caller: StoreContextMut<'_, T>, (span, ctx): (String, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.telemetry_emit(span, ctx)
                },
            )?;

            let mut http_iface = linker.instance("greentic:host-import/http@0.4.0")?;
            http_iface.func_wrap(
                "fetch",
                move |mut caller: StoreContextMut<'_, T>, (req, ctx): (http::HttpRequest, Option<types::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.http_fetch(req, ctx).map(|res| (res,))
                },
            )?;

            Ok(())
        }

        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:host-import@0.4.0";
    }
);

#[cfg(feature = "host-import-v0-2")]
declare_world!(
    mod host_import_v0_2,
    path = "wit/greentic/host-import@0.2.0",
    world = "greentic:host-import/host-imports@0.2.0",
    legacy = {
        use wasmtime::component::Linker;
        use wasmtime::{Result, StoreContextMut};

        pub use bindings::greentic::host_import::imports;

        /// Trait implemented by hosts to service the component imports.
        pub trait HostImports {
            fn secrets_get(
                &mut self,
                key: String,
                ctx: Option<imports::TenantCtx>,
            ) -> Result<Result<String, imports::IfaceError>>;

            fn telemetry_emit(
                &mut self,
                span_json: String,
                ctx: Option<imports::TenantCtx>,
            ) -> Result<()>;

            fn tool_invoke(
                &mut self,
                tool: String,
                action: String,
                args_json: String,
                ctx: Option<imports::TenantCtx>,
            ) -> Result<Result<String, imports::IfaceError>>;

            fn http_fetch(
                &mut self,
                req: imports::HttpRequest,
                ctx: Option<imports::TenantCtx>,
            ) -> Result<Result<imports::HttpResponse, imports::IfaceError>>;
        }

        /// Registers the host import functions with the provided linker.
        pub fn add_to_linker<T>(
            linker: &mut Linker<T>,
            get_host: impl Fn(&mut T) -> &mut (dyn HostImports + Send + Sync + 'static)
                + Send
                + Sync
                + Copy
                + 'static,
        ) -> Result<()>
        where
            T: Send + 'static,
        {
            let mut inst = linker.instance("greentic:host-import/host-imports@0.2.0")?;

            inst.func_wrap(
                "secrets-get",
                move |mut caller: StoreContextMut<'_, T>,
                      (key, ctx): (String, Option<imports::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.secrets_get(key, ctx).map(|res| (res,))
                },
            )?;

            inst.func_wrap(
                "telemetry-emit",
                move |mut caller: StoreContextMut<'_, T>,
                      (span, ctx): (String, Option<imports::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.telemetry_emit(span, ctx)
                },
            )?;

            inst.func_wrap(
                "tool-invoke",
                move |
                    mut caller: StoreContextMut<'_, T>,
                    (tool, action, args, ctx): (String, String, String, Option<imports::TenantCtx>),
                | {
                    let host = get_host(caller.data_mut());
                    host.tool_invoke(tool, action, args, ctx).map(|res| (res,))
                },
            )?;

            inst.func_wrap(
                "http-fetch",
                move |mut caller: StoreContextMut<'_, T>,
                      (req, ctx): (imports::HttpRequest, Option<imports::TenantCtx>)| {
                    let host = get_host(caller.data_mut());
                    host.http_fetch(req, ctx).map(|res| (res,))
                },
            )?;

            Ok(())
        }

        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:host-import@0.2.0";
    }
);

#[cfg(feature = "runner-host-v1")]
declare_world!(
    mod runner_host_v1,
    path = "wit/greentic/host@1.0.0",
    world = "greentic:host/runner-host@1.0.0",
    legacy = {
        use std::vec::Vec;
        use wasmtime::component::Linker;
        use wasmtime::{Result, StoreContextMut};

        pub use bindings::greentic::host::{http_v1, kv_v1, secrets_v1};

        /// Minimal trait hosts implement to satisfy the runner-host imports.
        pub trait RunnerHost {
            fn http_request(
                &mut self,
                method: String,
                url: String,
                headers: Vec<String>,
                body: Option<Vec<u8>>,
            ) -> Result<Result<Vec<u8>, String>>;

            fn secret_get(&mut self, name: String) -> Result<Result<String, String>>;

            fn kv_get(&mut self, ns: String, key: String) -> Result<Option<String>>;

            fn kv_put(&mut self, ns: String, key: String, val: String) -> Result<()>;
        }

        /// Registers the runner-host interfaces with the provided linker.
        pub fn add_to_linker<T>(
            linker: &mut Linker<T>,
            get_host: impl Fn(&mut T) -> &mut (dyn RunnerHost + Send + Sync + 'static)
                + Send
                + Sync
                + Copy
                + 'static,
        ) -> Result<()>
        where
            T: Send + 'static,
        {
            let mut http = linker.instance("greentic:host/http-v1@1.0.0")?;
            http.func_wrap(
                "request",
                move |mut caller: StoreContextMut<'_, T>,
                      (method, url, headers, body): (String, String, Vec<String>, Option<Vec<u8>>)| {
                    let host = get_host(caller.data_mut());
                    host.http_request(method, url, headers, body)
                        .map(|res| (res,))
                },
            )?;

            let mut secrets = linker.instance("greentic:host/secrets-v1@1.0.0")?;
            secrets.func_wrap(
                "get",
                move |mut caller: StoreContextMut<'_, T>, (name,): (String,)| {
                    let host = get_host(caller.data_mut());
                    host.secret_get(name).map(|res| (res,))
                },
            )?;

            let mut kv = linker.instance("greentic:host/kv-v1@1.0.0")?;
            kv.func_wrap(
                "get",
                move |mut caller: StoreContextMut<'_, T>, (ns, key): (String, String)| {
                    let host = get_host(caller.data_mut());
                    host.kv_get(ns, key).map(|res| (res,))
                },
            )?;
            kv.func_wrap(
                "put",
                move |mut caller: StoreContextMut<'_, T>, (ns, key, val): (String, String, String)| {
                    let host = get_host(caller.data_mut());
                    host.kv_put(ns, key, val)
                },
            )?;

            Ok(())
        }

        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:host@1.0.0";
    }
);

#[cfg(feature = "pack-export-v0-2")]
declare_world!(
    mod pack_export_v0_2,
    path = "wit/greentic/pack-export@0.2.0",
    world = "greentic:pack-export/pack-exports@0.2.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:pack-export@0.2.0";
    }
);

#[cfg(feature = "types-core-v0-2")]
declare_world!(
    mod types_core_v0_2,
    path = "wit/greentic/types-core@0.2.0",
    world = "greentic:types-core/core@0.2.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:types-core@0.2.0";
    }
);

#[cfg(feature = "secrets-v0-1")]
declare_world!(
    mod secrets_v0_1,
    path = "wit/greentic/secrets@0.1.0",
    world = "greentic:secrets/host@0.1.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:secrets@0.1.0";

        /// Raw WIT document for consumers that previously embedded it.
        pub const HOST_WORLD: &str =
            include_str!("../wit/greentic/secrets@0.1.0/package.wit");

        pub fn host_world() -> &'static str {
            HOST_WORLD
        }
    }
);

#[cfg(feature = "oauth-v0-1")]
declare_world!(
    mod oauth_v0_1,
    path = "wit/greentic/oauth@0.1.0",
    world = "greentic:oauth/oauth@0.1.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:oauth@0.1.0";
    }
);

#[cfg(feature = "oauth-broker-v1")]
declare_world!(
    mod oauth_broker_v1,
    path = "wit/greentic/oauth-broker@1.0.0",
    world = "greentic:oauth-broker/broker@1.0.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:oauth-broker@1.0.0";
    }
);

#[cfg(feature = "component-lifecycle-v1")]
declare_world!(
    mod component_lifecycle_v1,
    path = "wit/greentic/lifecycle@1.0.0",
    world = "greentic:lifecycle/component-lifecycle@1.0.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:lifecycle@1.0.0";
    }
);

#[cfg(feature = "events-v1")]
declare_world!(
    mod events_v1,
    path = "wit/greentic/events@1.0.0",
    world = "greentic:events/events@1.0.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:events@1.0.0";
    }
);

#[cfg(feature = "secrets-store-v1")]
declare_world!(
    mod secrets_store_v1,
    path = "wit/greentic/secrets-store@1.0.0",
    world = "greentic:secrets/store@1.0.0",
    legacy = {
        /// Canonical package identifier.
        pub const PACKAGE_ID: &str = "greentic:secrets@1.0.0";
    }
);

#[cfg(feature = "state-store-v1")]
declare_world!(
    mod state_store_v1,
    path = "wit/greentic/state-store@1.0.0",
    world = "greentic:state/store@1.0.0",
    legacy = {
        pub const PACKAGE_ID: &str = "greentic:state@1.0.0";
    }
);

#[cfg(feature = "messaging-session-v1")]
declare_world!(
    mod messaging_session_v1,
    path = "wit/greentic/messaging-session@1.0.0",
    world = "greentic:messaging/session@1.0.0",
    legacy = {
        pub const PACKAGE_ID: &str = "greentic:messaging@1.0.0";
    }
);

#[cfg(feature = "events-emitter-v1")]
declare_world!(
    mod events_emitter_v1,
    path = "wit/greentic/events-emitter@1.0.0",
    world = "greentic:events/emitter@1.0.0",
    legacy = {
        pub const PACKAGE_ID: &str = "greentic:events@1.0.0";
    }
);

#[cfg(feature = "http-client-v1")]
declare_world!(
    mod http_client_v1,
    path = "wit/greentic/http-client@1.0.0",
    world = "greentic:http/client@1.0.0",
    legacy = {
        pub const PACKAGE_ID: &str = "greentic:http@1.0.0";
    }
);

#[cfg(feature = "telemetry-logger-v1")]
declare_world!(
    mod telemetry_logger_v1,
    path = "wit/greentic/telemetry-logger@1.0.0",
    world = "greentic:telemetry/logger@1.0.0",
    legacy = {
        pub const PACKAGE_ID: &str = "greentic:telemetry@1.0.0";
    }
);
