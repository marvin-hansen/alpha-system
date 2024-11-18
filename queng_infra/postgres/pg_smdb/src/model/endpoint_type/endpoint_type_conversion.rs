use crate::model::endpoint_type::Endpoint;
use crate::model::protocol_type::ProtocolType;
use common_config::Endpoint as CommonEndpoint;

impl Endpoint {
    pub fn from_common_endpoint(endpoint: &CommonEndpoint) -> Self {
        Self {
            name: endpoint.name().to_string(),
            version: endpoint.version() as i32,
            base_uri: endpoint.uri().to_string(),
            port: endpoint.port() as i32,
            protocol: ProtocolType::from_common_protocol_type(&endpoint.protocol()),
        }
    }

    pub fn to_common_endpoint(&self) -> CommonEndpoint {
        CommonEndpoint::new(
            self.name.clone(),
            self.version as u32,
            self.base_uri.clone(),
            self.port as u32,
            self.protocol.to_common_protocol_type(),
        )
    }
}
