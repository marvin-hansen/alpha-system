/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::protocol_type::ProtocolType;
use common_config::ProtocolType as CommonProtocolType;

impl ProtocolType {
    pub const fn from_common_protocol_type(common_protocol_type: CommonProtocolType) -> Self {
        match common_protocol_type {
            CommonProtocolType::UnknownProtocol => Self::UnknownProtocol,
            CommonProtocolType::GRPC => Self::GRPC,
            CommonProtocolType::HTTP => Self::HTTP,
            CommonProtocolType::UDP => Self::UDP,
        }
    }

    pub const fn to_common_protocol_type(&self) -> CommonProtocolType {
        match self {
            Self::UnknownProtocol => CommonProtocolType::UnknownProtocol,
            Self::GRPC => CommonProtocolType::GRPC,
            Self::HTTP => CommonProtocolType::HTTP,
            Self::UDP => CommonProtocolType::UDP,
        }
    }
}
