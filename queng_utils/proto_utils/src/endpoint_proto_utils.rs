use std::fmt::Error;

use common_config::prelude::{Endpoint, ProtocolType};
use proto_bindings::proto::ProtoEndpoint;

/// Converts a `ProtoEndpoint` into an `Endpoint`.
///
/// This function takes a `ProtoEndpoint` and converts it into an `Endpoint` struct.
/// It extracts the necessary fields from the `ProtoEndpoint` and constructs a new `Endpoint` with them.
///
/// # Errors
///
/// If the `protocol` field of the `ProtoEndpoint` cannot be converted to a `ProtocolType`,
/// an `Error` is returned.
///
pub fn endpoint_from_proto(proto: ProtoEndpoint) -> Result<Endpoint, Error> {
    let protocol = ProtocolType::from(proto.protocol);

    Ok(Endpoint::new(
        proto.name.to_string(),
        proto.version,
        proto.uri.to_string(),
        proto.port,
        protocol,
    ))
}

/// Converts an `Endpoint` into a `ProtoEndpoint`.
///
/// This function takes an `Endpoint` and converts it into a `ProtoEndpoint` struct.
/// It extracts the necessary fields from the `Endpoint` and constructs a new `ProtoEndpoint` with them.
///
pub fn endpoint_to_proto(endpoint: Endpoint) -> Result<ProtoEndpoint, Error> {
    Ok(ProtoEndpoint {
        name: endpoint.name().to_string(),
        version: endpoint.version() as u32,
        uri: endpoint.uri().to_string(),
        port: endpoint.port() as u32,
        protocol: endpoint.protocol() as i32,
    })
}
