use common_data_bar::prelude::DataType;
use common_exchange::prelude::ExchangeID;
use proto_bindings::proto::{ProtoStartDataRequest, ProtoStopAllDataRequest, ProtoStopDataRequest};

pub(crate) fn get_start_data_request(
    exchange_id: ExchangeID,
    symbols: Vec<String>,
    data_type: DataType,
) -> ProtoStartDataRequest {
    ProtoStartDataRequest {
        exchange_id: exchange_id as u32,
        symbols,
        data_type_id: data_type as u32,
    }
}

pub(crate) fn get_stop_data_request(
    exchange_id: ExchangeID,
    stream_id: u32,
    data_type: DataType,
) -> ProtoStopDataRequest {
    ProtoStopDataRequest {
        exchange_id: exchange_id as u32,
        stream_id,
        data_type_id: data_type as u32,
    }
}

pub(crate) fn get_stop_all_data_request(exchange_id: ExchangeID) -> ProtoStopAllDataRequest {
    ProtoStopAllDataRequest {
        exchange_id: exchange_id as u32,
    }
}
