use common_metadata::prelude::MetaExchange;

use proto_mddb::proto::{
    CheckIfExchangeIdExistsRequest, CheckIfExchangeIdExistsResponse, CountExchangesResponse,
    GetAllExchangesRequest, GetExchangeRequest, LookupExchangeNameRequest,
};

// Request
pub fn get_check_if_exchange_exists_request(exchange_code: &str) -> CheckIfExchangeIdExistsRequest {
    CheckIfExchangeIdExistsRequest {
        exchange_code: exchange_code.to_string(),
    }
}

pub fn get_exchange_request(exchange_code: &str) -> GetExchangeRequest {
    GetExchangeRequest {
        exchange_code: exchange_code.to_string(),
    }
}

pub fn get_all_exchanges_request() -> GetAllExchangesRequest {
    GetAllExchangesRequest {}
}
pub fn get_lookup_exchange_name_request(exchange_code: &str) -> LookupExchangeNameRequest {
    LookupExchangeNameRequest {
        exchange_code: exchange_code.to_string(),
    }
}

// Response

pub fn get_count_exchanges_response(count: u64) -> CountExchangesResponse {
    CountExchangesResponse { count }
}

pub fn get_check_if_exchange_exists_response(
    exchange_code: &str,
    exists: bool,
) -> CheckIfExchangeIdExistsResponse {
    CheckIfExchangeIdExistsResponse {
        exchange_code: exchange_code.to_string(),
        exists,
    }
}

pub fn get_exchange_response(
    meta_exchange: &MetaExchange,
) -> proto_mddb::proto::GetExchangeResponse {
    proto_mddb::proto::GetExchangeResponse {
        exchange: Option::from(meta_exchange_to_proto_exchange(meta_exchange)),
    }
}

pub fn get_all_exchanges_response(
    meta_exchanges: Vec<MetaExchange>,
) -> proto_mddb::proto::GetAllExchangesResponse {
    proto_mddb::proto::GetAllExchangesResponse {
        exchanges: meta_exchanges
            .into_iter()
            .map(|meta_exchange: MetaExchange| meta_exchange_to_proto_exchange(&meta_exchange))
            .collect(),
    }
}

pub fn get_lookup_exchange_name_response(
    exchange_name: &str,
) -> proto_mddb::proto::LookupExchangeNameResponse {
    proto_mddb::proto::LookupExchangeNameResponse {
        exchange_name: exchange_name.to_string(),
    }
}

pub fn meta_exchange_to_proto_exchange(
    meta_exchange: &MetaExchange,
) -> proto_mddb::proto::ProtoMetaExchange {
    proto_mddb::proto::ProtoMetaExchange {
        exchange_code: meta_exchange.code.to_string(),
        exchange_name: meta_exchange.name.to_string(),
        exchange_hash: meta_exchange.hash(),
    }
}

pub fn proto_exchange_to_meta_exchange(
    proto_exchange: &proto_mddb::proto::ProtoMetaExchange,
) -> MetaExchange {
    MetaExchange {
        code: proto_exchange.exchange_code.to_string(),
        name: proto_exchange.exchange_name.to_string(),
        kaiko_legacy_slug: "".to_string(),
    }
}
