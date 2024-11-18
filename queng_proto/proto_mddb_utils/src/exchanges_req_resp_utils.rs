use crate::exchanges_utils;
use common_metadata::MetaExchange;
use proto_mddb::proto::{
    CheckIfExchangeIdExistsRequest, CheckIfExchangeIdExistsResponse, CountExchangesRequest,
    CountExchangesResponse, GetAllExchangesRequest, GetExchangeRequest, LookupExchangeNameRequest,
};

// Request
pub fn get_count_exchanges_request() -> CountExchangesRequest {
    CountExchangesRequest {}
}

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

pub fn get_check_if_exchange_exists_response(exists: bool) -> CheckIfExchangeIdExistsResponse {
    CheckIfExchangeIdExistsResponse { exists }
}

pub fn get_exchange_response(
    meta_exchange: Option<MetaExchange>,
) -> proto_mddb::proto::GetExchangeResponse {
    if let Some(exchanges) = meta_exchange {
        proto_mddb::proto::GetExchangeResponse {
            exchange: Option::from(exchanges_utils::meta_exchange_to_proto_exchange(&exchanges)),
        }
    } else {
        proto_mddb::proto::GetExchangeResponse { exchange: None }
    }
}

pub fn get_all_exchanges_response(
    meta_exchanges: Vec<MetaExchange>,
) -> proto_mddb::proto::GetAllExchangesResponse {
    proto_mddb::proto::GetAllExchangesResponse {
        exchanges: meta_exchanges
            .into_iter()
            .map(|meta_exchange: MetaExchange| {
                exchanges_utils::meta_exchange_to_proto_exchange(&meta_exchange)
            })
            .collect(),
    }
}

pub fn get_lookup_exchange_name_response(
    exchange: Option<MetaExchange>,
) -> proto_mddb::proto::LookupExchangeNameResponse {
    if let Some(exchange) = exchange {
        proto_mddb::proto::LookupExchangeNameResponse {
            exchange_name: Some(exchange.name.to_string()),
        }
    } else {
        proto_mddb::proto::LookupExchangeNameResponse {
            exchange_name: None,
        }
    }
}
