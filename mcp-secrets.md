# <a id="mcp_secrets"></a>World mcp-secrets

world mcp {
  Exporting the router interface as part of the world
  export router;
}
world secrets {
  import secrets-store;
  export secrets-list;
}

 - Imports:
    - interface `wasix:mcp/secrets-store@0.0.5`
    - interface `wasi:io/poll@0.2.2`
    - interface `wasi:clocks/monotonic-clock@0.2.2`
    - interface `wasi:io/error@0.2.2`
    - interface `wasi:io/streams@0.2.2`
    - interface `wasi:http/types@0.2.2`
    - interface `wasi:http/outgoing-handler@0.2.2`
    - interface `wasi:logging/logging@0.1.0-draft`
 - Exports:
    - interface `wasix:mcp/router@0.0.5`
    - interface `wasix:mcp/secrets-list@0.0.5`

## <a id="wasix_mcp_secrets_store_0_0_5"></a>Import interface wasix:mcp/secrets-store@0.0.5


----

### Types

#### <a id="secrets_error"></a>`variant secrets-error`

An error type that encapsulates the different errors that can occur fetching secrets

##### Variant Cases

- <a id="secrets_error.upstream"></a>`upstream`: `string`
  <p>This indicates an error from an "upstream" secrets source.
  As this could be almost _anything_ (such as Vault, Kubernetes Secrets, KeyValue buckets, etc),
  the error message is a string.

- <a id="secrets_error.io"></a>`io`: `string`
  <p>This indicates an error from an I/O operation.
  As this could be almost _anything_ (such as a file read, network connection, etc),
  the error message is a string.
  Depending on how this ends up being consumed,
  we may consider moving this to use the `wasi:io/error` type instead.
  For simplicity right now in supporting multiple implementations, it is being left as a string.

- <a id="secrets_error.not_found"></a>`not-found`
  <p>This indicates that the secret was not found. Generally "not found" errors will
  be handled by the upstream secrets backend, but there are cases where the host
  may need to return this error.

#### <a id="secret_value"></a>`record secret-value`

A secret value.

##### Record Fields

- <a id="secret_value.secret"></a>`secret`: `string`
  <p>A string value

#### <a id="secret"></a>`resource secret`

A secret is a resource that can only be borrowed. This allows you to
pass around handles to secrets and not reveal the values until a
component needs them.
You need to use the reveal interface to get the value.
----

### Functions

#### <a id="get"></a>`get: func`

Gets a single opaque secrets value set at the given key if it exists

##### Params

- <a id="get.key"></a>`key`: `string`

##### Return values

- <a id="get.0"></a> result<own<[`secret`](#secret)>, [`secrets-error`](#secrets_error)>

#### <a id="reveal"></a>`reveal: func`


##### Params

- <a id="reveal.s"></a>`s`: borrow<[`secret`](#secret)>

##### Return values

- <a id="reveal.0"></a> [`secret-value`](#secret_value)

## <a id="wasi_io_poll_0_2_2"></a>Import interface wasi:io/poll@0.2.2


----

### Types

#### <a id="pollable"></a>`resource pollable`

----

### Functions

#### <a id="method_pollable_ready"></a>`[method]pollable.ready: func`


##### Params

- <a id="method_pollable_ready.self"></a>`self`: borrow<[`pollable`](#pollable)>

##### Return values

- <a id="method_pollable_ready.0"></a> `bool`

#### <a id="method_pollable_block"></a>`[method]pollable.block: func`


##### Params

- <a id="method_pollable_block.self"></a>`self`: borrow<[`pollable`](#pollable)>

#### <a id="poll"></a>`poll: func`


##### Params

- <a id="poll.in"></a>`in`: list<borrow<[`pollable`](#pollable)>>

##### Return values

- <a id="poll.0"></a> list<`u32`>

## <a id="wasi_clocks_monotonic_clock_0_2_2"></a>Import interface wasi:clocks/monotonic-clock@0.2.2


----

### Types

#### <a id="pollable"></a>`type pollable`
[`pollable`](#pollable)
<p>
#### <a id="instant"></a>`type instant`
`u64`
<p>
#### <a id="duration"></a>`type duration`
`u64`
<p>
----

### Functions

#### <a id="now"></a>`now: func`


##### Return values

- <a id="now.0"></a> [`instant`](#instant)

#### <a id="resolution"></a>`resolution: func`


##### Return values

- <a id="resolution.0"></a> [`duration`](#duration)

#### <a id="subscribe_instant"></a>`subscribe-instant: func`


##### Params

- <a id="subscribe_instant.when"></a>`when`: [`instant`](#instant)

##### Return values

- <a id="subscribe_instant.0"></a> own<[`pollable`](#pollable)>

#### <a id="subscribe_duration"></a>`subscribe-duration: func`


##### Params

- <a id="subscribe_duration.when"></a>`when`: [`duration`](#duration)

##### Return values

- <a id="subscribe_duration.0"></a> own<[`pollable`](#pollable)>

## <a id="wasi_io_error_0_2_2"></a>Import interface wasi:io/error@0.2.2


----

### Types

#### <a id="error"></a>`resource error`

----

### Functions

#### <a id="method_error_to_debug_string"></a>`[method]error.to-debug-string: func`


##### Params

- <a id="method_error_to_debug_string.self"></a>`self`: borrow<[`error`](#error)>

##### Return values

- <a id="method_error_to_debug_string.0"></a> `string`

## <a id="wasi_io_streams_0_2_2"></a>Import interface wasi:io/streams@0.2.2


----

### Types

#### <a id="error"></a>`type error`
[`error`](#error)
<p>
#### <a id="pollable"></a>`type pollable`
[`pollable`](#pollable)
<p>
#### <a id="stream_error"></a>`variant stream-error`


##### Variant Cases

- <a id="stream_error.last_operation_failed"></a>`last-operation-failed`: own<[`error`](#error)>
- <a id="stream_error.closed"></a>`closed`
#### <a id="input_stream"></a>`resource input-stream`

#### <a id="output_stream"></a>`resource output-stream`

----

### Functions

#### <a id="method_input_stream_read"></a>`[method]input-stream.read: func`


##### Params

- <a id="method_input_stream_read.self"></a>`self`: borrow<[`input-stream`](#input_stream)>
- <a id="method_input_stream_read.len"></a>`len`: `u64`

##### Return values

- <a id="method_input_stream_read.0"></a> result<list<`u8`>, [`stream-error`](#stream_error)>

#### <a id="method_input_stream_blocking_read"></a>`[method]input-stream.blocking-read: func`


##### Params

- <a id="method_input_stream_blocking_read.self"></a>`self`: borrow<[`input-stream`](#input_stream)>
- <a id="method_input_stream_blocking_read.len"></a>`len`: `u64`

##### Return values

- <a id="method_input_stream_blocking_read.0"></a> result<list<`u8`>, [`stream-error`](#stream_error)>

#### <a id="method_input_stream_skip"></a>`[method]input-stream.skip: func`


##### Params

- <a id="method_input_stream_skip.self"></a>`self`: borrow<[`input-stream`](#input_stream)>
- <a id="method_input_stream_skip.len"></a>`len`: `u64`

##### Return values

- <a id="method_input_stream_skip.0"></a> result<`u64`, [`stream-error`](#stream_error)>

#### <a id="method_input_stream_blocking_skip"></a>`[method]input-stream.blocking-skip: func`


##### Params

- <a id="method_input_stream_blocking_skip.self"></a>`self`: borrow<[`input-stream`](#input_stream)>
- <a id="method_input_stream_blocking_skip.len"></a>`len`: `u64`

##### Return values

- <a id="method_input_stream_blocking_skip.0"></a> result<`u64`, [`stream-error`](#stream_error)>

#### <a id="method_input_stream_subscribe"></a>`[method]input-stream.subscribe: func`


##### Params

- <a id="method_input_stream_subscribe.self"></a>`self`: borrow<[`input-stream`](#input_stream)>

##### Return values

- <a id="method_input_stream_subscribe.0"></a> own<[`pollable`](#pollable)>

#### <a id="method_output_stream_check_write"></a>`[method]output-stream.check-write: func`


##### Params

- <a id="method_output_stream_check_write.self"></a>`self`: borrow<[`output-stream`](#output_stream)>

##### Return values

- <a id="method_output_stream_check_write.0"></a> result<`u64`, [`stream-error`](#stream_error)>

#### <a id="method_output_stream_write"></a>`[method]output-stream.write: func`


##### Params

- <a id="method_output_stream_write.self"></a>`self`: borrow<[`output-stream`](#output_stream)>
- <a id="method_output_stream_write.contents"></a>`contents`: list<`u8`>

##### Return values

- <a id="method_output_stream_write.0"></a> result<_, [`stream-error`](#stream_error)>

#### <a id="method_output_stream_blocking_write_and_flush"></a>`[method]output-stream.blocking-write-and-flush: func`


##### Params

- <a id="method_output_stream_blocking_write_and_flush.self"></a>`self`: borrow<[`output-stream`](#output_stream)>
- <a id="method_output_stream_blocking_write_and_flush.contents"></a>`contents`: list<`u8`>

##### Return values

- <a id="method_output_stream_blocking_write_and_flush.0"></a> result<_, [`stream-error`](#stream_error)>

#### <a id="method_output_stream_flush"></a>`[method]output-stream.flush: func`


##### Params

- <a id="method_output_stream_flush.self"></a>`self`: borrow<[`output-stream`](#output_stream)>

##### Return values

- <a id="method_output_stream_flush.0"></a> result<_, [`stream-error`](#stream_error)>

#### <a id="method_output_stream_blocking_flush"></a>`[method]output-stream.blocking-flush: func`


##### Params

- <a id="method_output_stream_blocking_flush.self"></a>`self`: borrow<[`output-stream`](#output_stream)>

##### Return values

- <a id="method_output_stream_blocking_flush.0"></a> result<_, [`stream-error`](#stream_error)>

#### <a id="method_output_stream_subscribe"></a>`[method]output-stream.subscribe: func`


##### Params

- <a id="method_output_stream_subscribe.self"></a>`self`: borrow<[`output-stream`](#output_stream)>

##### Return values

- <a id="method_output_stream_subscribe.0"></a> own<[`pollable`](#pollable)>

#### <a id="method_output_stream_write_zeroes"></a>`[method]output-stream.write-zeroes: func`


##### Params

- <a id="method_output_stream_write_zeroes.self"></a>`self`: borrow<[`output-stream`](#output_stream)>
- <a id="method_output_stream_write_zeroes.len"></a>`len`: `u64`

##### Return values

- <a id="method_output_stream_write_zeroes.0"></a> result<_, [`stream-error`](#stream_error)>

#### <a id="method_output_stream_blocking_write_zeroes_and_flush"></a>`[method]output-stream.blocking-write-zeroes-and-flush: func`


##### Params

- <a id="method_output_stream_blocking_write_zeroes_and_flush.self"></a>`self`: borrow<[`output-stream`](#output_stream)>
- <a id="method_output_stream_blocking_write_zeroes_and_flush.len"></a>`len`: `u64`

##### Return values

- <a id="method_output_stream_blocking_write_zeroes_and_flush.0"></a> result<_, [`stream-error`](#stream_error)>

#### <a id="method_output_stream_splice"></a>`[method]output-stream.splice: func`


##### Params

- <a id="method_output_stream_splice.self"></a>`self`: borrow<[`output-stream`](#output_stream)>
- <a id="method_output_stream_splice.src"></a>`src`: borrow<[`input-stream`](#input_stream)>
- <a id="method_output_stream_splice.len"></a>`len`: `u64`

##### Return values

- <a id="method_output_stream_splice.0"></a> result<`u64`, [`stream-error`](#stream_error)>

#### <a id="method_output_stream_blocking_splice"></a>`[method]output-stream.blocking-splice: func`


##### Params

- <a id="method_output_stream_blocking_splice.self"></a>`self`: borrow<[`output-stream`](#output_stream)>
- <a id="method_output_stream_blocking_splice.src"></a>`src`: borrow<[`input-stream`](#input_stream)>
- <a id="method_output_stream_blocking_splice.len"></a>`len`: `u64`

##### Return values

- <a id="method_output_stream_blocking_splice.0"></a> result<`u64`, [`stream-error`](#stream_error)>

## <a id="wasi_http_types_0_2_2"></a>Import interface wasi:http/types@0.2.2

This interface defines all of the types and methods for implementing
HTTP Requests and Responses, both incoming and outgoing, as well as
their headers, trailers, and bodies.

----

### Types

#### <a id="duration"></a>`type duration`
[`duration`](#duration)
<p>
#### <a id="input_stream"></a>`type input-stream`
[`input-stream`](#input_stream)
<p>
#### <a id="output_stream"></a>`type output-stream`
[`output-stream`](#output_stream)
<p>
#### <a id="io_error"></a>`type io-error`
[`error`](#error)
<p>
#### <a id="pollable"></a>`type pollable`
[`pollable`](#pollable)
<p>
#### <a id="method"></a>`variant method`

This type corresponds to HTTP standard Methods.

##### Variant Cases

- <a id="method.get"></a>`get`
- <a id="method.head"></a>`head`
- <a id="method.post"></a>`post`
- <a id="method.put"></a>`put`
- <a id="method.delete"></a>`delete`
- <a id="method.connect"></a>`connect`
- <a id="method.options"></a>`options`
- <a id="method.trace"></a>`trace`
- <a id="method.patch"></a>`patch`
- <a id="method.other"></a>`other`: `string`
#### <a id="scheme"></a>`variant scheme`

This type corresponds to HTTP standard Related Schemes.

##### Variant Cases

- <a id="scheme.http"></a>`HTTP`
- <a id="scheme.https"></a>`HTTPS`
- <a id="scheme.other"></a>`other`: `string`
#### <a id="dns_error_payload"></a>`record DNS-error-payload`

Defines the case payload type for `DNS-error` above:

##### Record Fields

- <a id="dns_error_payload.rcode"></a>`rcode`: option<`string`>
- <a id="dns_error_payload.info_code"></a>`info-code`: option<`u16`>
#### <a id="tls_alert_received_payload"></a>`record TLS-alert-received-payload`

Defines the case payload type for `TLS-alert-received` above:

##### Record Fields

- <a id="tls_alert_received_payload.alert_id"></a>`alert-id`: option<`u8`>
- <a id="tls_alert_received_payload.alert_message"></a>`alert-message`: option<`string`>
#### <a id="field_size_payload"></a>`record field-size-payload`

Defines the case payload type for `HTTP-response-{header,trailer}-size` above:

##### Record Fields

- <a id="field_size_payload.field_name"></a>`field-name`: option<`string`>
- <a id="field_size_payload.field_size"></a>`field-size`: option<`u32`>
#### <a id="error_code"></a>`variant error-code`

These cases are inspired by the IANA HTTP Proxy Error Types:
https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types

##### Variant Cases

- <a id="error_code.dns_timeout"></a>`DNS-timeout`
- <a id="error_code.dns_error"></a>`DNS-error`: [`DNS-error-payload`](#dns_error_payload)
- <a id="error_code.destination_not_found"></a>`destination-not-found`
- <a id="error_code.destination_unavailable"></a>`destination-unavailable`
- <a id="error_code.destination_ip_prohibited"></a>`destination-IP-prohibited`
- <a id="error_code.destination_ip_unroutable"></a>`destination-IP-unroutable`
- <a id="error_code.connection_refused"></a>`connection-refused`
- <a id="error_code.connection_terminated"></a>`connection-terminated`
- <a id="error_code.connection_timeout"></a>`connection-timeout`
- <a id="error_code.connection_read_timeout"></a>`connection-read-timeout`
- <a id="error_code.connection_write_timeout"></a>`connection-write-timeout`
- <a id="error_code.connection_limit_reached"></a>`connection-limit-reached`
- <a id="error_code.tls_protocol_error"></a>`TLS-protocol-error`
- <a id="error_code.tls_certificate_error"></a>`TLS-certificate-error`
- <a id="error_code.tls_alert_received"></a>`TLS-alert-received`: [`TLS-alert-received-payload`](#tls_alert_received_payload)
- <a id="error_code.http_request_denied"></a>`HTTP-request-denied`
- <a id="error_code.http_request_length_required"></a>`HTTP-request-length-required`
- <a id="error_code.http_request_body_size"></a>`HTTP-request-body-size`: option<`u64`>
- <a id="error_code.http_request_method_invalid"></a>`HTTP-request-method-invalid`
- <a id="error_code.http_request_uri_invalid"></a>`HTTP-request-URI-invalid`
- <a id="error_code.http_request_uri_too_long"></a>`HTTP-request-URI-too-long`
- <a id="error_code.http_request_header_section_size"></a>`HTTP-request-header-section-size`: option<`u32`>
- <a id="error_code.http_request_header_size"></a>`HTTP-request-header-size`: option<[`field-size-payload`](#field_size_payload)>
- <a id="error_code.http_request_trailer_section_size"></a>`HTTP-request-trailer-section-size`: option<`u32`>
- <a id="error_code.http_request_trailer_size"></a>`HTTP-request-trailer-size`: [`field-size-payload`](#field_size_payload)
- <a id="error_code.http_response_incomplete"></a>`HTTP-response-incomplete`
- <a id="error_code.http_response_header_section_size"></a>`HTTP-response-header-section-size`: option<`u32`>
- <a id="error_code.http_response_header_size"></a>`HTTP-response-header-size`: [`field-size-payload`](#field_size_payload)
- <a id="error_code.http_response_body_size"></a>`HTTP-response-body-size`: option<`u64`>
- <a id="error_code.http_response_trailer_section_size"></a>`HTTP-response-trailer-section-size`: option<`u32`>
- <a id="error_code.http_response_trailer_size"></a>`HTTP-response-trailer-size`: [`field-size-payload`](#field_size_payload)
- <a id="error_code.http_response_transfer_coding"></a>`HTTP-response-transfer-coding`: option<`string`>
- <a id="error_code.http_response_content_coding"></a>`HTTP-response-content-coding`: option<`string`>
- <a id="error_code.http_response_timeout"></a>`HTTP-response-timeout`
- <a id="error_code.http_upgrade_failed"></a>`HTTP-upgrade-failed`
- <a id="error_code.http_protocol_error"></a>`HTTP-protocol-error`
- <a id="error_code.loop_detected"></a>`loop-detected`
- <a id="error_code.configuration_error"></a>`configuration-error`
- <a id="error_code.internal_error"></a>`internal-error`: option<`string`>
  <p>This is a catch-all error for anything that doesn't fit cleanly into a
  more specific case. It also includes an optional string for an
  unstructured description of the error. Users should not depend on the
  string for diagnosing errors, as it's not required to be consistent
  between implementations.

#### <a id="header_error"></a>`variant header-error`

This type enumerates the different kinds of errors that may occur when
setting or appending to a `fields` resource.

##### Variant Cases

- <a id="header_error.invalid_syntax"></a>`invalid-syntax`
  <p>This error indicates that a `field-name` or `field-value` was
  syntactically invalid when used with an operation that sets headers in a
  `fields`.

- <a id="header_error.forbidden"></a>`forbidden`
  <p>This error indicates that a forbidden `field-name` was used when trying
  to set a header in a `fields`.

- <a id="header_error.immutable"></a>`immutable`
  <p>This error indicates that the operation on the `fields` was not
  permitted because the fields are immutable.

#### <a id="field_key"></a>`type field-key`
`string`
<p>Field keys are always strings.

Field keys should always be treated as case insensitive by the `fields`
resource for the purposes of equality checking.

# Deprecation

This type has been deprecated in favor of the `field-name` type.

#### <a id="field_name"></a>`type field-name`
[`field-key`](#field_key)
<p>Field names are always strings.

Field names should always be treated as case insensitive by the `fields`
resource for the purposes of equality checking.

#### <a id="field_value"></a>`type field-value`
[`field-value`](#field_value)
<p>Field values should always be ASCII strings. However, in
reality, HTTP implementations often have to interpret malformed values,
so they are provided as a list of bytes.

#### <a id="fields"></a>`resource fields`

This following block defines the `fields` resource which corresponds to
HTTP standard Fields. Fields are a common representation used for both
Headers and Trailers.

A `fields` may be mutable or immutable. A `fields` created using the
constructor, `from-list`, or `clone` will be mutable, but a `fields`
resource given by other means (including, but not limited to,
`incoming-request.headers`, `outgoing-request.headers`) might be be
immutable. In an immutable fields, the `set`, `append`, and `delete`
operations will fail with `header-error.immutable`.
#### <a id="headers"></a>`type headers`
[`fields`](#fields)
<p>Headers is an alias for Fields.

#### <a id="trailers"></a>`type trailers`
[`fields`](#fields)
<p>Trailers is an alias for Fields.

#### <a id="incoming_request"></a>`resource incoming-request`

Represents an incoming HTTP Request.
#### <a id="outgoing_request"></a>`resource outgoing-request`

Represents an outgoing HTTP Request.
#### <a id="request_options"></a>`resource request-options`

Parameters for making an HTTP Request. Each of these parameters is
currently an optional timeout applicable to the transport layer of the
HTTP protocol.

These timeouts are separate from any the user may use to bound a
blocking call to `wasi:io/poll.poll`.
#### <a id="response_outparam"></a>`resource response-outparam`

Represents the ability to send an HTTP Response.

This resource is used by the `wasi:http/incoming-handler` interface to
allow a Response to be sent corresponding to the Request provided as the
other argument to `incoming-handler.handle`.
#### <a id="status_code"></a>`type status-code`
`u16`
<p>This type corresponds to the HTTP standard Status Code.

#### <a id="incoming_response"></a>`resource incoming-response`

Represents an incoming HTTP Response.
#### <a id="incoming_body"></a>`resource incoming-body`

Represents an incoming HTTP Request or Response's Body.

A body has both its contents - a stream of bytes - and a (possibly
empty) set of trailers, indicating that the full contents of the
body have been received. This resource represents the contents as
an `input-stream` and the delivery of trailers as a `future-trailers`,
and ensures that the user of this interface may only be consuming either
the body contents or waiting on trailers at any given time.
#### <a id="future_trailers"></a>`resource future-trailers`

Represents a future which may eventually return trailers, or an error.

In the case that the incoming HTTP Request or Response did not have any
trailers, this future will resolve to the empty set of trailers once the
complete Request or Response body has been received.
#### <a id="outgoing_response"></a>`resource outgoing-response`

Represents an outgoing HTTP Response.
#### <a id="outgoing_body"></a>`resource outgoing-body`

Represents an outgoing HTTP Request or Response's Body.

A body has both its contents - a stream of bytes - and a (possibly
empty) set of trailers, inducating the full contents of the body
have been sent. This resource represents the contents as an
`output-stream` child resource, and the completion of the body (with
optional trailers) with a static function that consumes the
`outgoing-body` resource, and ensures that the user of this interface
may not write to the body contents after the body has been finished.

If the user code drops this resource, as opposed to calling the static
method `finish`, the implementation should treat the body as incomplete,
and that an error has occurred. The implementation should propagate this
error to the HTTP protocol by whatever means it has available,
including: corrupting the body on the wire, aborting the associated
Request, or sending a late status code for the Response.
#### <a id="future_incoming_response"></a>`resource future-incoming-response`

Represents a future which may eventually return an incoming HTTP
Response, or an error.

This resource is returned by the `wasi:http/outgoing-handler` interface to
provide the HTTP Response corresponding to the sent Request.
----

### Functions

#### <a id="constructor_fields"></a>`[constructor]fields: func`

Construct an empty HTTP Fields.

The resulting `fields` is mutable.

##### Return values

- <a id="constructor_fields.0"></a> own<[`fields`](#fields)>

#### <a id="static_fields_from_list"></a>`[static]fields.from-list: func`

Construct an HTTP Fields.

The resulting `fields` is mutable.

The list represents each name-value pair in the Fields. Names
which have multiple values are represented by multiple entries in this
list with the same name.

The tuple is a pair of the field name, represented as a string, and
Value, represented as a list of bytes.

An error result will be returned if any `field-name` or `field-value` is
syntactically invalid, or if a field is forbidden.

##### Params

- <a id="static_fields_from_list.entries"></a>`entries`: list<([`field-name`](#field_name), [`field-value`](#field_value))>

##### Return values

- <a id="static_fields_from_list.0"></a> result<own<[`fields`](#fields)>, [`header-error`](#header_error)>

#### <a id="method_fields_get"></a>`[method]fields.get: func`

Get all of the values corresponding to a name. If the name is not present
in this `fields` or is syntactically invalid, an empty list is returned.
However, if the name is present but empty, this is represented by a list
with one or more empty field-values present.

##### Params

- <a id="method_fields_get.self"></a>`self`: borrow<[`fields`](#fields)>
- <a id="method_fields_get.name"></a>`name`: [`field-name`](#field_name)

##### Return values

- <a id="method_fields_get.0"></a> list<[`field-value`](#field_value)>

#### <a id="method_fields_has"></a>`[method]fields.has: func`

Returns `true` when the name is present in this `fields`. If the name is
syntactically invalid, `false` is returned.

##### Params

- <a id="method_fields_has.self"></a>`self`: borrow<[`fields`](#fields)>
- <a id="method_fields_has.name"></a>`name`: [`field-name`](#field_name)

##### Return values

- <a id="method_fields_has.0"></a> `bool`

#### <a id="method_fields_set"></a>`[method]fields.set: func`

Set all of the values for a name. Clears any existing values for that
name, if they have been set.

Fails with `header-error.immutable` if the `fields` are immutable.

Fails with `header-error.invalid-syntax` if the `field-name` or any of
the `field-value`s are syntactically invalid.

##### Params

- <a id="method_fields_set.self"></a>`self`: borrow<[`fields`](#fields)>
- <a id="method_fields_set.name"></a>`name`: [`field-name`](#field_name)
- <a id="method_fields_set.value"></a>`value`: list<[`field-value`](#field_value)>

##### Return values

- <a id="method_fields_set.0"></a> result<_, [`header-error`](#header_error)>

#### <a id="method_fields_delete"></a>`[method]fields.delete: func`

Delete all values for a name. Does nothing if no values for the name
exist.

Fails with `header-error.immutable` if the `fields` are immutable.

Fails with `header-error.invalid-syntax` if the `field-name` is
syntactically invalid.

##### Params

- <a id="method_fields_delete.self"></a>`self`: borrow<[`fields`](#fields)>
- <a id="method_fields_delete.name"></a>`name`: [`field-name`](#field_name)

##### Return values

- <a id="method_fields_delete.0"></a> result<_, [`header-error`](#header_error)>

#### <a id="method_fields_append"></a>`[method]fields.append: func`

Append a value for a name. Does not change or delete any existing
values for that name.

Fails with `header-error.immutable` if the `fields` are immutable.

Fails with `header-error.invalid-syntax` if the `field-name` or
`field-value` are syntactically invalid.

##### Params

- <a id="method_fields_append.self"></a>`self`: borrow<[`fields`](#fields)>
- <a id="method_fields_append.name"></a>`name`: [`field-name`](#field_name)
- <a id="method_fields_append.value"></a>`value`: [`field-value`](#field_value)

##### Return values

- <a id="method_fields_append.0"></a> result<_, [`header-error`](#header_error)>

#### <a id="method_fields_entries"></a>`[method]fields.entries: func`

Retrieve the full set of names and values in the Fields. Like the
constructor, the list represents each name-value pair.

The outer list represents each name-value pair in the Fields. Names
which have multiple values are represented by multiple entries in this
list with the same name.

The names and values are always returned in the original casing and in
the order in which they will be serialized for transport.

##### Params

- <a id="method_fields_entries.self"></a>`self`: borrow<[`fields`](#fields)>

##### Return values

- <a id="method_fields_entries.0"></a> list<([`field-name`](#field_name), [`field-value`](#field_value))>

#### <a id="method_fields_clone"></a>`[method]fields.clone: func`

Make a deep copy of the Fields. Equivalent in behavior to calling the
`fields` constructor on the return value of `entries`. The resulting
`fields` is mutable.

##### Params

- <a id="method_fields_clone.self"></a>`self`: borrow<[`fields`](#fields)>

##### Return values

- <a id="method_fields_clone.0"></a> own<[`fields`](#fields)>

#### <a id="method_incoming_request_method"></a>`[method]incoming-request.method: func`

Returns the method of the incoming request.

##### Params

- <a id="method_incoming_request_method.self"></a>`self`: borrow<[`incoming-request`](#incoming_request)>

##### Return values

- <a id="method_incoming_request_method.0"></a> [`method`](#method)

#### <a id="method_incoming_request_path_with_query"></a>`[method]incoming-request.path-with-query: func`

Returns the path with query parameters from the request, as a string.

##### Params

- <a id="method_incoming_request_path_with_query.self"></a>`self`: borrow<[`incoming-request`](#incoming_request)>

##### Return values

- <a id="method_incoming_request_path_with_query.0"></a> option<`string`>

#### <a id="method_incoming_request_scheme"></a>`[method]incoming-request.scheme: func`

Returns the protocol scheme from the request.

##### Params

- <a id="method_incoming_request_scheme.self"></a>`self`: borrow<[`incoming-request`](#incoming_request)>

##### Return values

- <a id="method_incoming_request_scheme.0"></a> option<[`scheme`](#scheme)>

#### <a id="method_incoming_request_authority"></a>`[method]incoming-request.authority: func`

Returns the authority of the Request's target URI, if present.

##### Params

- <a id="method_incoming_request_authority.self"></a>`self`: borrow<[`incoming-request`](#incoming_request)>

##### Return values

- <a id="method_incoming_request_authority.0"></a> option<`string`>

#### <a id="method_incoming_request_headers"></a>`[method]incoming-request.headers: func`

Get the `headers` associated with the request.

The returned `headers` resource is immutable: `set`, `append`, and
`delete` operations will fail with `header-error.immutable`.

The `headers` returned are a child resource: it must be dropped before
the parent `incoming-request` is dropped. Dropping this
`incoming-request` before all children are dropped will trap.

##### Params

- <a id="method_incoming_request_headers.self"></a>`self`: borrow<[`incoming-request`](#incoming_request)>

##### Return values

- <a id="method_incoming_request_headers.0"></a> own<[`headers`](#headers)>

#### <a id="method_incoming_request_consume"></a>`[method]incoming-request.consume: func`

Gives the `incoming-body` associated with this request. Will only
return success at most once, and subsequent calls will return error.

##### Params

- <a id="method_incoming_request_consume.self"></a>`self`: borrow<[`incoming-request`](#incoming_request)>

##### Return values

- <a id="method_incoming_request_consume.0"></a> result<own<[`incoming-body`](#incoming_body)>>

#### <a id="constructor_outgoing_request"></a>`[constructor]outgoing-request: func`

Construct a new `outgoing-request` with a default `method` of `GET`, and
`none` values for `path-with-query`, `scheme`, and `authority`.

* `headers` is the HTTP Headers for the Request.

It is possible to construct, or manipulate with the accessor functions
below, an `outgoing-request` with an invalid combination of `scheme`
and `authority`, or `headers` which are not permitted to be sent.
It is the obligation of the `outgoing-handler.handle` implementation
to reject invalid constructions of `outgoing-request`.

##### Params

- <a id="constructor_outgoing_request.headers"></a>`headers`: own<[`headers`](#headers)>

##### Return values

- <a id="constructor_outgoing_request.0"></a> own<[`outgoing-request`](#outgoing_request)>

#### <a id="method_outgoing_request_body"></a>`[method]outgoing-request.body: func`

Returns the resource corresponding to the outgoing Body for this
Request.

Returns success on the first call: the `outgoing-body` resource for
this `outgoing-request` can be retrieved at most once. Subsequent
calls will return error.

##### Params

- <a id="method_outgoing_request_body.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>

##### Return values

- <a id="method_outgoing_request_body.0"></a> result<own<[`outgoing-body`](#outgoing_body)>>

#### <a id="method_outgoing_request_method"></a>`[method]outgoing-request.method: func`

Get the Method for the Request.

##### Params

- <a id="method_outgoing_request_method.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>

##### Return values

- <a id="method_outgoing_request_method.0"></a> [`method`](#method)

#### <a id="method_outgoing_request_set_method"></a>`[method]outgoing-request.set-method: func`

Set the Method for the Request. Fails if the string present in a
`method.other` argument is not a syntactically valid method.

##### Params

- <a id="method_outgoing_request_set_method.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>
- <a id="method_outgoing_request_set_method.method"></a>`method`: [`method`](#method)

##### Return values

- <a id="method_outgoing_request_set_method.0"></a> result

#### <a id="method_outgoing_request_path_with_query"></a>`[method]outgoing-request.path-with-query: func`

Get the combination of the HTTP Path and Query for the Request.
When `none`, this represents an empty Path and empty Query.

##### Params

- <a id="method_outgoing_request_path_with_query.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>

##### Return values

- <a id="method_outgoing_request_path_with_query.0"></a> option<`string`>

#### <a id="method_outgoing_request_set_path_with_query"></a>`[method]outgoing-request.set-path-with-query: func`

Set the combination of the HTTP Path and Query for the Request.
When `none`, this represents an empty Path and empty Query. Fails is the
string given is not a syntactically valid path and query uri component.

##### Params

- <a id="method_outgoing_request_set_path_with_query.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>
- <a id="method_outgoing_request_set_path_with_query.path_with_query"></a>`path-with-query`: option<`string`>

##### Return values

- <a id="method_outgoing_request_set_path_with_query.0"></a> result

#### <a id="method_outgoing_request_scheme"></a>`[method]outgoing-request.scheme: func`

Get the HTTP Related Scheme for the Request. When `none`, the
implementation may choose an appropriate default scheme.

##### Params

- <a id="method_outgoing_request_scheme.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>

##### Return values

- <a id="method_outgoing_request_scheme.0"></a> option<[`scheme`](#scheme)>

#### <a id="method_outgoing_request_set_scheme"></a>`[method]outgoing-request.set-scheme: func`

Set the HTTP Related Scheme for the Request. When `none`, the
implementation may choose an appropriate default scheme. Fails if the
string given is not a syntactically valid uri scheme.

##### Params

- <a id="method_outgoing_request_set_scheme.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>
- <a id="method_outgoing_request_set_scheme.scheme"></a>`scheme`: option<[`scheme`](#scheme)>

##### Return values

- <a id="method_outgoing_request_set_scheme.0"></a> result

#### <a id="method_outgoing_request_authority"></a>`[method]outgoing-request.authority: func`

Get the authority of the Request's target URI. A value of `none` may be used
with Related Schemes which do not require an authority. The HTTP and
HTTPS schemes always require an authority.

##### Params

- <a id="method_outgoing_request_authority.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>

##### Return values

- <a id="method_outgoing_request_authority.0"></a> option<`string`>

#### <a id="method_outgoing_request_set_authority"></a>`[method]outgoing-request.set-authority: func`

Set the authority of the Request's target URI. A value of `none` may be used
with Related Schemes which do not require an authority. The HTTP and
HTTPS schemes always require an authority. Fails if the string given is
not a syntactically valid URI authority.

##### Params

- <a id="method_outgoing_request_set_authority.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>
- <a id="method_outgoing_request_set_authority.authority"></a>`authority`: option<`string`>

##### Return values

- <a id="method_outgoing_request_set_authority.0"></a> result

#### <a id="method_outgoing_request_headers"></a>`[method]outgoing-request.headers: func`

Get the headers associated with the Request.

The returned `headers` resource is immutable: `set`, `append`, and
`delete` operations will fail with `header-error.immutable`.

This headers resource is a child: it must be dropped before the parent
`outgoing-request` is dropped, or its ownership is transferred to
another component by e.g. `outgoing-handler.handle`.

##### Params

- <a id="method_outgoing_request_headers.self"></a>`self`: borrow<[`outgoing-request`](#outgoing_request)>

##### Return values

- <a id="method_outgoing_request_headers.0"></a> own<[`headers`](#headers)>

#### <a id="constructor_request_options"></a>`[constructor]request-options: func`

Construct a default `request-options` value.

##### Return values

- <a id="constructor_request_options.0"></a> own<[`request-options`](#request_options)>

#### <a id="method_request_options_connect_timeout"></a>`[method]request-options.connect-timeout: func`

The timeout for the initial connect to the HTTP Server.

##### Params

- <a id="method_request_options_connect_timeout.self"></a>`self`: borrow<[`request-options`](#request_options)>

##### Return values

- <a id="method_request_options_connect_timeout.0"></a> option<[`duration`](#duration)>

#### <a id="method_request_options_set_connect_timeout"></a>`[method]request-options.set-connect-timeout: func`

Set the timeout for the initial connect to the HTTP Server. An error
return value indicates that this timeout is not supported.

##### Params

- <a id="method_request_options_set_connect_timeout.self"></a>`self`: borrow<[`request-options`](#request_options)>
- <a id="method_request_options_set_connect_timeout.duration"></a>`duration`: option<[`duration`](#duration)>

##### Return values

- <a id="method_request_options_set_connect_timeout.0"></a> result

#### <a id="method_request_options_first_byte_timeout"></a>`[method]request-options.first-byte-timeout: func`

The timeout for receiving the first byte of the Response body.

##### Params

- <a id="method_request_options_first_byte_timeout.self"></a>`self`: borrow<[`request-options`](#request_options)>

##### Return values

- <a id="method_request_options_first_byte_timeout.0"></a> option<[`duration`](#duration)>

#### <a id="method_request_options_set_first_byte_timeout"></a>`[method]request-options.set-first-byte-timeout: func`

Set the timeout for receiving the first byte of the Response body. An
error return value indicates that this timeout is not supported.

##### Params

- <a id="method_request_options_set_first_byte_timeout.self"></a>`self`: borrow<[`request-options`](#request_options)>
- <a id="method_request_options_set_first_byte_timeout.duration"></a>`duration`: option<[`duration`](#duration)>

##### Return values

- <a id="method_request_options_set_first_byte_timeout.0"></a> result

#### <a id="method_request_options_between_bytes_timeout"></a>`[method]request-options.between-bytes-timeout: func`

The timeout for receiving subsequent chunks of bytes in the Response
body stream.

##### Params

- <a id="method_request_options_between_bytes_timeout.self"></a>`self`: borrow<[`request-options`](#request_options)>

##### Return values

- <a id="method_request_options_between_bytes_timeout.0"></a> option<[`duration`](#duration)>

#### <a id="method_request_options_set_between_bytes_timeout"></a>`[method]request-options.set-between-bytes-timeout: func`

Set the timeout for receiving subsequent chunks of bytes in the Response
body stream. An error return value indicates that this timeout is not
supported.

##### Params

- <a id="method_request_options_set_between_bytes_timeout.self"></a>`self`: borrow<[`request-options`](#request_options)>
- <a id="method_request_options_set_between_bytes_timeout.duration"></a>`duration`: option<[`duration`](#duration)>

##### Return values

- <a id="method_request_options_set_between_bytes_timeout.0"></a> result

#### <a id="static_response_outparam_set"></a>`[static]response-outparam.set: func`

Set the value of the `response-outparam` to either send a response,
or indicate an error.

This method consumes the `response-outparam` to ensure that it is
called at most once. If it is never called, the implementation
will respond with an error.

The user may provide an `error` to `response` to allow the
implementation determine how to respond with an HTTP error response.

##### Params

- <a id="static_response_outparam_set.param"></a>`param`: own<[`response-outparam`](#response_outparam)>
- <a id="static_response_outparam_set.response"></a>`response`: result<own<[`outgoing-response`](#outgoing_response)>, [`error-code`](#error_code)>

#### <a id="method_incoming_response_status"></a>`[method]incoming-response.status: func`

Returns the status code from the incoming response.

##### Params

- <a id="method_incoming_response_status.self"></a>`self`: borrow<[`incoming-response`](#incoming_response)>

##### Return values

- <a id="method_incoming_response_status.0"></a> [`status-code`](#status_code)

#### <a id="method_incoming_response_headers"></a>`[method]incoming-response.headers: func`

Returns the headers from the incoming response.

The returned `headers` resource is immutable: `set`, `append`, and
`delete` operations will fail with `header-error.immutable`.

This headers resource is a child: it must be dropped before the parent
`incoming-response` is dropped.

##### Params

- <a id="method_incoming_response_headers.self"></a>`self`: borrow<[`incoming-response`](#incoming_response)>

##### Return values

- <a id="method_incoming_response_headers.0"></a> own<[`headers`](#headers)>

#### <a id="method_incoming_response_consume"></a>`[method]incoming-response.consume: func`

Returns the incoming body. May be called at most once. Returns error
if called additional times.

##### Params

- <a id="method_incoming_response_consume.self"></a>`self`: borrow<[`incoming-response`](#incoming_response)>

##### Return values

- <a id="method_incoming_response_consume.0"></a> result<own<[`incoming-body`](#incoming_body)>>

#### <a id="method_incoming_body_stream"></a>`[method]incoming-body.stream: func`

Returns the contents of the body, as a stream of bytes.

Returns success on first call: the stream representing the contents
can be retrieved at most once. Subsequent calls will return error.

The returned `input-stream` resource is a child: it must be dropped
before the parent `incoming-body` is dropped, or consumed by
`incoming-body.finish`.

This invariant ensures that the implementation can determine whether
the user is consuming the contents of the body, waiting on the
`future-trailers` to be ready, or neither. This allows for network
backpressure is to be applied when the user is consuming the body,
and for that backpressure to not inhibit delivery of the trailers if
the user does not read the entire body.

##### Params

- <a id="method_incoming_body_stream.self"></a>`self`: borrow<[`incoming-body`](#incoming_body)>

##### Return values

- <a id="method_incoming_body_stream.0"></a> result<own<[`input-stream`](#input_stream)>>

#### <a id="static_incoming_body_finish"></a>`[static]incoming-body.finish: func`

Takes ownership of `incoming-body`, and returns a `future-trailers`.
This function will trap if the `input-stream` child is still alive.

##### Params

- <a id="static_incoming_body_finish.this"></a>`this`: own<[`incoming-body`](#incoming_body)>

##### Return values

- <a id="static_incoming_body_finish.0"></a> own<[`future-trailers`](#future_trailers)>

#### <a id="method_future_trailers_subscribe"></a>`[method]future-trailers.subscribe: func`

Returns a pollable which becomes ready when either the trailers have
been received, or an error has occurred. When this pollable is ready,
the `get` method will return `some`.

##### Params

- <a id="method_future_trailers_subscribe.self"></a>`self`: borrow<[`future-trailers`](#future_trailers)>

##### Return values

- <a id="method_future_trailers_subscribe.0"></a> own<[`pollable`](#pollable)>

#### <a id="method_future_trailers_get"></a>`[method]future-trailers.get: func`

Returns the contents of the trailers, or an error which occurred,
once the future is ready.

The outer `option` represents future readiness. Users can wait on this
`option` to become `some` using the `subscribe` method.

The outer `result` is used to retrieve the trailers or error at most
once. It will be success on the first call in which the outer option
is `some`, and error on subsequent calls.

The inner `result` represents that either the HTTP Request or Response
body, as well as any trailers, were received successfully, or that an
error occurred receiving them. The optional `trailers` indicates whether
or not trailers were present in the body.

When some `trailers` are returned by this method, the `trailers`
resource is immutable, and a child. Use of the `set`, `append`, or
`delete` methods will return an error, and the resource must be
dropped before the parent `future-trailers` is dropped.

##### Params

- <a id="method_future_trailers_get.self"></a>`self`: borrow<[`future-trailers`](#future_trailers)>

##### Return values

- <a id="method_future_trailers_get.0"></a> option<result<result<option<own<[`trailers`](#trailers)>>, [`error-code`](#error_code)>>>

#### <a id="constructor_outgoing_response"></a>`[constructor]outgoing-response: func`

Construct an `outgoing-response`, with a default `status-code` of `200`.
If a different `status-code` is needed, it must be set via the
`set-status-code` method.

* `headers` is the HTTP Headers for the Response.

##### Params

- <a id="constructor_outgoing_response.headers"></a>`headers`: own<[`headers`](#headers)>

##### Return values

- <a id="constructor_outgoing_response.0"></a> own<[`outgoing-response`](#outgoing_response)>

#### <a id="method_outgoing_response_status_code"></a>`[method]outgoing-response.status-code: func`

Get the HTTP Status Code for the Response.

##### Params

- <a id="method_outgoing_response_status_code.self"></a>`self`: borrow<[`outgoing-response`](#outgoing_response)>

##### Return values

- <a id="method_outgoing_response_status_code.0"></a> [`status-code`](#status_code)

#### <a id="method_outgoing_response_set_status_code"></a>`[method]outgoing-response.set-status-code: func`

Set the HTTP Status Code for the Response. Fails if the status-code
given is not a valid http status code.

##### Params

- <a id="method_outgoing_response_set_status_code.self"></a>`self`: borrow<[`outgoing-response`](#outgoing_response)>
- <a id="method_outgoing_response_set_status_code.status_code"></a>`status-code`: [`status-code`](#status_code)

##### Return values

- <a id="method_outgoing_response_set_status_code.0"></a> result

#### <a id="method_outgoing_response_headers"></a>`[method]outgoing-response.headers: func`

Get the headers associated with the Request.

The returned `headers` resource is immutable: `set`, `append`, and
`delete` operations will fail with `header-error.immutable`.

This headers resource is a child: it must be dropped before the parent
`outgoing-request` is dropped, or its ownership is transferred to
another component by e.g. `outgoing-handler.handle`.

##### Params

- <a id="method_outgoing_response_headers.self"></a>`self`: borrow<[`outgoing-response`](#outgoing_response)>

##### Return values

- <a id="method_outgoing_response_headers.0"></a> own<[`headers`](#headers)>

#### <a id="method_outgoing_response_body"></a>`[method]outgoing-response.body: func`

Returns the resource corresponding to the outgoing Body for this Response.

Returns success on the first call: the `outgoing-body` resource for
this `outgoing-response` can be retrieved at most once. Subsequent
calls will return error.

##### Params

- <a id="method_outgoing_response_body.self"></a>`self`: borrow<[`outgoing-response`](#outgoing_response)>

##### Return values

- <a id="method_outgoing_response_body.0"></a> result<own<[`outgoing-body`](#outgoing_body)>>

#### <a id="method_outgoing_body_write"></a>`[method]outgoing-body.write: func`

Returns a stream for writing the body contents.

The returned `output-stream` is a child resource: it must be dropped
before the parent `outgoing-body` resource is dropped (or finished),
otherwise the `outgoing-body` drop or `finish` will trap.

Returns success on the first call: the `output-stream` resource for
this `outgoing-body` may be retrieved at most once. Subsequent calls
will return error.

##### Params

- <a id="method_outgoing_body_write.self"></a>`self`: borrow<[`outgoing-body`](#outgoing_body)>

##### Return values

- <a id="method_outgoing_body_write.0"></a> result<own<[`output-stream`](#output_stream)>>

#### <a id="static_outgoing_body_finish"></a>`[static]outgoing-body.finish: func`

Finalize an outgoing body, optionally providing trailers. This must be
called to signal that the response is complete. If the `outgoing-body`
is dropped without calling `outgoing-body.finalize`, the implementation
should treat the body as corrupted.

Fails if the body's `outgoing-request` or `outgoing-response` was
constructed with a Content-Length header, and the contents written
to the body (via `write`) does not match the value given in the
Content-Length.

##### Params

- <a id="static_outgoing_body_finish.this"></a>`this`: own<[`outgoing-body`](#outgoing_body)>
- <a id="static_outgoing_body_finish.trailers"></a>`trailers`: option<own<[`trailers`](#trailers)>>

##### Return values

- <a id="static_outgoing_body_finish.0"></a> result<_, [`error-code`](#error_code)>

#### <a id="method_future_incoming_response_subscribe"></a>`[method]future-incoming-response.subscribe: func`

Returns a pollable which becomes ready when either the Response has
been received, or an error has occurred. When this pollable is ready,
the `get` method will return `some`.

##### Params

- <a id="method_future_incoming_response_subscribe.self"></a>`self`: borrow<[`future-incoming-response`](#future_incoming_response)>

##### Return values

- <a id="method_future_incoming_response_subscribe.0"></a> own<[`pollable`](#pollable)>

#### <a id="method_future_incoming_response_get"></a>`[method]future-incoming-response.get: func`

Returns the incoming HTTP Response, or an error, once one is ready.

The outer `option` represents future readiness. Users can wait on this
`option` to become `some` using the `subscribe` method.

The outer `result` is used to retrieve the response or error at most
once. It will be success on the first call in which the outer option
is `some`, and error on subsequent calls.

The inner `result` represents that either the incoming HTTP Response
status and headers have received successfully, or that an error
occurred. Errors may also occur while consuming the response body,
but those will be reported by the `incoming-body` and its
`output-stream` child.

##### Params

- <a id="method_future_incoming_response_get.self"></a>`self`: borrow<[`future-incoming-response`](#future_incoming_response)>

##### Return values

- <a id="method_future_incoming_response_get.0"></a> option<result<result<own<[`incoming-response`](#incoming_response)>, [`error-code`](#error_code)>>>

#### <a id="http_error_code"></a>`http-error-code: func`

Attempts to extract a http-related `error` from the wasi:io `error`
provided.

Stream operations which return
`wasi:io/stream/stream-error::last-operation-failed` have a payload of
type `wasi:io/error/error` with more information about the operation
that failed. This payload can be passed through to this function to see
if there's http-related information about the error to return.

Note that this function is fallible because not all io-errors are
http-related errors.

##### Params

- <a id="http_error_code.err"></a>`err`: borrow<[`io-error`](#io_error)>

##### Return values

- <a id="http_error_code.0"></a> option<[`error-code`](#error_code)>

## <a id="wasi_http_outgoing_handler_0_2_2"></a>Import interface wasi:http/outgoing-handler@0.2.2

This interface defines a handler of outgoing HTTP Requests. It should be
imported by components which wish to make HTTP Requests.

----

### Types

#### <a id="outgoing_request"></a>`type outgoing-request`
[`outgoing-request`](#outgoing_request)
<p>
#### <a id="request_options"></a>`type request-options`
[`request-options`](#request_options)
<p>
#### <a id="future_incoming_response"></a>`type future-incoming-response`
[`future-incoming-response`](#future_incoming_response)
<p>
#### <a id="error_code"></a>`type error-code`
[`error-code`](#error_code)
<p>
----

### Functions

#### <a id="handle"></a>`handle: func`

This function is invoked with an outgoing HTTP Request, and it returns
a resource `future-incoming-response` which represents an HTTP Response
which may arrive in the future.

The `options` argument accepts optional parameters for the HTTP
protocol's transport layer.

This function may return an error if the `outgoing-request` is invalid
or not allowed to be made. Otherwise, protocol errors are reported
through the `future-incoming-response`.

##### Params

- <a id="handle.request"></a>`request`: own<[`outgoing-request`](#outgoing_request)>
- <a id="handle.options"></a>`options`: option<own<[`request-options`](#request_options)>>

##### Return values

- <a id="handle.0"></a> result<own<[`future-incoming-response`](#future_incoming_response)>, [`error-code`](#error_code)>

## <a id="wasi_logging_logging_0_1_0_draft"></a>Import interface wasi:logging/logging@0.1.0-draft

WASI Logging is a logging API intended to let users emit log messages with
simple priority levels and context values.

----

### Types

#### <a id="level"></a>`enum level`

A log level, describing a kind of message.

##### Enum Cases

- <a id="level.trace"></a>`trace`
  <p>Describes messages about the values of variables and the flow of
  control within a program.

- <a id="level.debug"></a>`debug`
  <p>Describes messages likely to be of interest to someone debugging a
  program.

- <a id="level.info"></a>`info`
  <p>Describes messages likely to be of interest to someone monitoring a
  program.

- <a id="level.warn"></a>`warn`
  <p>Describes messages indicating hazardous situations.

- <a id="level.error"></a>`error`
  <p>Describes messages indicating serious errors.

- <a id="level.critical"></a>`critical`
  <p>Describes messages indicating fatal errors.

----

### Functions

#### <a id="log"></a>`log: func`

Emit a log message.

A log message has a `level` describing what kind of message is being
sent, a context, which is an uninterpreted string meant to help
consumers group similar messages, and a string containing the message
text.

##### Params

- <a id="log.level"></a>`level`: [`level`](#level)
- <a id="log.context"></a>`context`: `string`
- <a id="log.message"></a>`message`: `string`

## <a id="wasix_mcp_router_0_0_5"></a>Export interface wasix:mcp/router@0.0.5

----

### Types

#### <a id="value"></a>`record value`


##### Record Fields

- <a id="value.json"></a>`json`: `string`
#### <a id="tool"></a>`record tool`


##### Record Fields

- <a id="tool.name"></a>`name`: `string`
- <a id="tool.description"></a>`description`: `string`
- <a id="tool.input_schema"></a>`input-schema`: [`value`](#value)
- <a id="tool.output_schema"></a>`output-schema`: option<[`value`](#value)>
#### <a id="prompts_capability"></a>`record prompts-capability`

Prompts capability (list_changed field)

##### Record Fields

- <a id="prompts_capability.list_changed"></a>`list-changed`: option<`bool`>
#### <a id="resources_capability"></a>`record resources-capability`

Resources capability (subscribe and list_changed fields)

##### Record Fields

- <a id="resources_capability.subscribe"></a>`subscribe`: option<`bool`>
- <a id="resources_capability.list_changed"></a>`list-changed`: option<`bool`>
#### <a id="tools_capability"></a>`record tools-capability`

Tools capability (list_changed field)

##### Record Fields

- <a id="tools_capability.list_changed"></a>`list-changed`: option<`bool`>
#### <a id="server_capabilities"></a>`record server-capabilities`


##### Record Fields

- <a id="server_capabilities.prompts"></a>`prompts`: option<[`prompts-capability`](#prompts_capability)>
- <a id="server_capabilities.resources"></a>`resources`: option<[`resources-capability`](#resources_capability)>
- <a id="server_capabilities.tools"></a>`tools`: option<[`tools-capability`](#tools_capability)>
#### <a id="text_resource_contents"></a>`record text-resource-contents`


##### Record Fields

- <a id="text_resource_contents.uri"></a>`uri`: `string`
- <a id="text_resource_contents.mime_type"></a>`mime-type`: option<`string`>
- <a id="text_resource_contents.text"></a>`text`: `string`
#### <a id="blob_resource_contents"></a>`record blob-resource-contents`


##### Record Fields

- <a id="blob_resource_contents.uri"></a>`uri`: `string`
- <a id="blob_resource_contents.mime_type"></a>`mime-type`: option<`string`>
- <a id="blob_resource_contents.blob"></a>`blob`: `string`
#### <a id="resource_contents"></a>`variant resource-contents`


##### Variant Cases

- <a id="resource_contents.text"></a>`text`: [`text-resource-contents`](#text_resource_contents)
- <a id="resource_contents.blob"></a>`blob`: [`blob-resource-contents`](#blob_resource_contents)
#### <a id="read_resource_result"></a>`record read-resource-result`


##### Record Fields

- <a id="read_resource_result.contents"></a>`contents`: list<[`resource-contents`](#resource_contents)>
#### <a id="role"></a>`enum role`


##### Enum Cases

- <a id="role.user"></a>`user`
- <a id="role.assistant"></a>`assistant`
#### <a id="datetime"></a>`type datetime`
`string`
<p>
#### <a id="annotations"></a>`record annotations`


##### Record Fields

- <a id="annotations.audience"></a>`audience`: option<list<[`role`](#role)>>
- <a id="annotations.priority"></a>`priority`: option<`f32`>
- <a id="annotations.timestamp"></a>`timestamp`: option<[`datetime`](#datetime)>
#### <a id="text_content"></a>`record text-content`


##### Record Fields

- <a id="text_content.text"></a>`text`: `string`
- <a id="text_content.annotations"></a>`annotations`: option<[`annotations`](#annotations)>
#### <a id="image_content"></a>`record image-content`


##### Record Fields

- <a id="image_content.data"></a>`data`: `string`
- <a id="image_content.mime_type"></a>`mime-type`: `string`
- <a id="image_content.annotations"></a>`annotations`: option<[`annotations`](#annotations)>
#### <a id="embedded_resource"></a>`record embedded-resource`


##### Record Fields

- <a id="embedded_resource.resource_contents"></a>`resource-contents`: [`resource-contents`](#resource_contents)
- <a id="embedded_resource.annotations"></a>`annotations`: option<[`annotations`](#annotations)>
#### <a id="content"></a>`variant content`


##### Variant Cases

- <a id="content.text"></a>`text`: [`text-content`](#text_content)
- <a id="content.image"></a>`image`: [`image-content`](#image_content)
- <a id="content.embedded"></a>`embedded`: [`embedded-resource`](#embedded_resource)
#### <a id="call_tool_result"></a>`record call-tool-result`


##### Record Fields

- <a id="call_tool_result.content"></a>`content`: list<[`content`](#content)>
- <a id="call_tool_result.is_error"></a>`is-error`: option<`bool`>
#### <a id="mcp_resource"></a>`record mcp-resource`


##### Record Fields

- <a id="mcp_resource.uri"></a>`uri`: `string`
- <a id="mcp_resource.name"></a>`name`: `string`
- <a id="mcp_resource.description"></a>`description`: option<`string`>
- <a id="mcp_resource.mime_type"></a>`mime-type`: `string`
- <a id="mcp_resource.annotations"></a>`annotations`: option<[`annotations`](#annotations)>
#### <a id="tool_error"></a>`variant tool-error`

Define as string for UTC datetime
Errors

##### Variant Cases

- <a id="tool_error.invalid_parameters"></a>`invalid-parameters`: `string`
- <a id="tool_error.execution_error"></a>`execution-error`: `string`
- <a id="tool_error.schema_error"></a>`schema-error`: `string`
- <a id="tool_error.not_found"></a>`not-found`: `string`
#### <a id="resource_error"></a>`variant resource-error`


##### Variant Cases

- <a id="resource_error.execution_error"></a>`execution-error`: `string`
- <a id="resource_error.not_found"></a>`not-found`: `string`
#### <a id="prompt_error"></a>`variant prompt-error`


##### Variant Cases

- <a id="prompt_error.invalid_parameters"></a>`invalid-parameters`: `string`
- <a id="prompt_error.internal_error"></a>`internal-error`: `string`
- <a id="prompt_error.not_found"></a>`not-found`: `string`
#### <a id="prompt_argument"></a>`record prompt-argument`


##### Record Fields

- <a id="prompt_argument.name"></a>`name`: `string`
- <a id="prompt_argument.description"></a>`description`: option<`string`>
- <a id="prompt_argument.required"></a>`required`: option<`bool`>
#### <a id="prompt"></a>`record prompt`


##### Record Fields

- <a id="prompt.name"></a>`name`: `string`
- <a id="prompt.description"></a>`description`: option<`string`>
- <a id="prompt.arguments"></a>`arguments`: option<list<[`prompt-argument`](#prompt_argument)>>
#### <a id="prompt_message_role"></a>`enum prompt-message-role`


##### Enum Cases

- <a id="prompt_message_role.user"></a>`user`
- <a id="prompt_message_role.assistant"></a>`assistant`
#### <a id="prompt_message_content"></a>`variant prompt-message-content`


##### Variant Cases

- <a id="prompt_message_content.text"></a>`text`: [`text-content`](#text_content)
- <a id="prompt_message_content.image"></a>`image`: [`image-content`](#image_content)
- <a id="prompt_message_content.mcp_resource"></a>`mcp-resource`: [`embedded-resource`](#embedded_resource)
#### <a id="prompt_message"></a>`record prompt-message`


##### Record Fields

- <a id="prompt_message.role"></a>`role`: [`prompt-message-role`](#prompt_message_role)
- <a id="prompt_message.content"></a>`content`: [`prompt-message-content`](#prompt_message_content)
#### <a id="get_prompt_result"></a>`record get-prompt-result`


##### Record Fields

- <a id="get_prompt_result.description"></a>`description`: option<`string`>
- <a id="get_prompt_result.messages"></a>`messages`: list<[`prompt-message`](#prompt_message)>
----

### Functions

#### <a id="name"></a>`name: func`


##### Return values

- <a id="name.0"></a> `string`

#### <a id="instructions"></a>`instructions: func`


##### Return values

- <a id="instructions.0"></a> `string`

#### <a id="capabilities"></a>`capabilities: func`


##### Return values

- <a id="capabilities.0"></a> [`server-capabilities`](#server_capabilities)

#### <a id="list_tools"></a>`list-tools: func`


##### Return values

- <a id="list_tools.0"></a> list<[`tool`](#tool)>

#### <a id="call_tool"></a>`call-tool: func`


##### Params

- <a id="call_tool.tool_name"></a>`tool-name`: `string`
- <a id="call_tool.arguments"></a>`arguments`: [`value`](#value)

##### Return values

- <a id="call_tool.0"></a> result<[`call-tool-result`](#call_tool_result), [`tool-error`](#tool_error)>

#### <a id="list_resources"></a>`list-resources: func`


##### Return values

- <a id="list_resources.0"></a> list<[`mcp-resource`](#mcp_resource)>

#### <a id="read_resource"></a>`read-resource: func`


##### Params

- <a id="read_resource.uri"></a>`uri`: `string`

##### Return values

- <a id="read_resource.0"></a> result<[`read-resource-result`](#read_resource_result), [`resource-error`](#resource_error)>

#### <a id="list_prompts"></a>`list-prompts: func`


##### Return values

- <a id="list_prompts.0"></a> list<[`prompt`](#prompt)>

#### <a id="get_prompt"></a>`get-prompt: func`


##### Params

- <a id="get_prompt.prompt_name"></a>`prompt-name`: `string`

##### Return values

- <a id="get_prompt.0"></a> result<[`get-prompt-result`](#get_prompt_result), [`prompt-error`](#prompt_error)>

## <a id="wasix_mcp_secrets_list_0_0_5"></a>Export interface wasix:mcp/secrets-list@0.0.5

----

### Types

#### <a id="secrets_description"></a>`record secrets-description`


##### Record Fields

- <a id="secrets_description.name"></a>`name`: `string`
- <a id="secrets_description.description"></a>`description`: `string`
- <a id="secrets_description.required"></a>`required`: `bool`
----

### Functions

#### <a id="list_secrets"></a>`list-secrets: func`


##### Return values

- <a id="list_secrets.0"></a> list<[`secrets-description`](#secrets_description)>

