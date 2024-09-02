use std::fmt::Error;

use common_config::prelude::{Endpoint, ProtocolType};
use proto_smdb::proto::ProtoEndpoint;

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
pub fn endpoint_from_proto(proto: Vec<ProtoEndpoint>) -> Result<Vec<Endpoint>, Error> {
    let mut endpoints = Vec::new();

    for proto_endpoint in proto {
        let protocol = ProtocolType::from(proto_endpoint.protocol);
        let endpoint = Endpoint::new(
            proto_endpoint.name.to_string(),
            proto_endpoint.version,
            proto_endpoint.uri.to_string(),
            proto_endpoint.port,
            protocol,
        );
        endpoints.push(endpoint);
    }

    Ok(endpoints)
}

/// Converts an `Endpoint` into a `ProtoEndpoint`.
///
/// This function takes an `Endpoint` and converts it into a `ProtoEndpoint` struct.
/// It extracts the necessary fields from the `Endpoint` and constructs a new `ProtoEndpoint` with them.
///
pub fn endpoint_to_proto(endpoints: &Vec<Endpoint>) -> Result<Vec<ProtoEndpoint>, Error> {
    let mut proto_endpoints = Vec::new();

    for endpoint in endpoints {
        let proto_endpoint = ProtoEndpoint {
            name: endpoint.name().to_string(),
            version: endpoint.version(),
            uri: endpoint.uri().to_string(),
            port: endpoint.port(),
            protocol: endpoint.protocol() as i32,
        };
        proto_endpoints.push(proto_endpoint);
    }

    Ok(proto_endpoints)
}
