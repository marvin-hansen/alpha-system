use crate::model::protocol_type::ProtocolType;
use common_config::ProtocolType as CommonProtocolType;

impl ProtocolType {
    pub fn from_common_protocol_type(common_protocol_type: &CommonProtocolType) -> Self {
        match common_protocol_type {
            CommonProtocolType::UnknownProtocol => ProtocolType::UnknownProtocol,
            CommonProtocolType::GRPC => ProtocolType::GRPC,
            CommonProtocolType::HTTP => ProtocolType::HTTP,
            CommonProtocolType::UDP => ProtocolType::UDP,
        }
    }

    pub fn to_common_protocol_type(&self) -> CommonProtocolType {
        match self {
            ProtocolType::UnknownProtocol => CommonProtocolType::UnknownProtocol,
            ProtocolType::GRPC => CommonProtocolType::GRPC,
            ProtocolType::HTTP => CommonProtocolType::HTTP,
            ProtocolType::UDP => CommonProtocolType::UDP,
        }
    }
}
