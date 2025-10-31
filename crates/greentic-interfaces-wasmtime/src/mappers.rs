use std::convert::TryFrom;

use greentic_interfaces::bindings::greentic::interfaces_types::types as abi_types;
use greentic_types as t;

/// Converts a `TenantCtx` from `greentic-types` into the ABI representation.
pub fn map_tenant_to_abi(tenant: &t::TenantCtx) -> Result<abi_types::TenantCtx, t::GreenticError> {
    abi_types::TenantCtx::try_from(tenant.clone())
}

/// Converts an ABI `TenantCtx` back into the rich Greentic type.
pub fn map_tenant_from_abi(tenant: abi_types::TenantCtx) -> Result<t::TenantCtx, t::GreenticError> {
    t::TenantCtx::try_from(tenant)
}

/// Maps an ABI outcome into the Greentic `Outcome<String>` representation.
pub fn map_outcome_from_abi(outcome: abi_types::Outcome) -> t::Outcome<String> {
    outcome.into()
}

/// Maps a Greentic outcome into the ABI structure.
pub fn map_outcome_to_abi(outcome: t::Outcome<String>) -> abi_types::Outcome {
    outcome.into()
}

/// Converts a span context from greentic-types into the ABI representation.
pub fn map_span_to_abi(span: &t::SpanContext) -> Result<abi_types::SpanContext, t::GreenticError> {
    abi_types::SpanContext::try_from(span.clone())
}

/// Converts an ABI span context into the greentic-types struct.
pub fn map_span_from_abi(span: abi_types::SpanContext) -> Result<t::SpanContext, t::GreenticError> {
    t::SpanContext::try_from(span)
}

/// Converts a session cursor into the ABI representation.
pub fn map_session_cursor_to_abi(cursor: t::SessionCursor) -> abi_types::SessionCursor {
    cursor.into()
}

/// Converts an ABI session cursor into the Greentic type.
pub fn map_session_cursor_from_abi(cursor: abi_types::SessionCursor) -> t::SessionCursor {
    cursor.into()
}
