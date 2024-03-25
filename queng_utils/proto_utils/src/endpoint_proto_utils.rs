use common::prelude::{Encoding, Endpoint, ProtocolType};
use proto_bindings::proto::ProtoEndpoint;
use std::fmt::Error;

pub fn endpoint_from_proto(proto: ProtoEndpoint) -> Result<Endpoint, Error> {
    let protocol = ProtocolType::from(proto.protocol);
    let encoding = Encoding::from(proto.encoding);

    Ok(Endpoint::new(
        proto.name.to_string(),
        proto.version as u8,
        proto.description.to_string(),
        proto.uri.to_string(),
        proto.port as u16,
        protocol,
        encoding,
    ))
}

pub fn endpoint_to_proto(endpoint: Endpoint) -> Result<ProtoEndpoint, Error> {
    Ok(ProtoEndpoint {
        name: endpoint.name().to_string(),
        version: endpoint.version() as u32,
        description: endpoint.description().to_string(),
        uri: endpoint.uri().to_string(),
        port: endpoint.port() as u32,
        protocol: endpoint.protocol() as i32,
        encoding: endpoint.encoding() as i32,
    })
}
