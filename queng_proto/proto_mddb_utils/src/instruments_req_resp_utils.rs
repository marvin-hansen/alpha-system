use crate::instruments_utils;
use common_metadata::prelude::MetaInstrument;

// Request
pub fn get_count_instruments_request() -> proto_mddb::proto::CountInstrumentsRequest {
    proto_mddb::proto::CountInstrumentsRequest {}
}

pub fn get_check_if_instrument_exists_request(
    instrument_id: &str,
) -> proto_mddb::proto::CheckIfInstrumentIdExistsRequest {
    proto_mddb::proto::CheckIfInstrumentIdExistsRequest {
        instrument_id: instrument_id.to_string(),
    }
}

pub fn get_instrument_by_id_request(
    instrument_id: &str,
) -> proto_mddb::proto::GetInstrumentByIdRequest {
    proto_mddb::proto::GetInstrumentByIdRequest {
        instrument_id: instrument_id.to_string(),
    }
}

pub fn get_instrument_by_figi_request(
    instrument_figi: &str,
) -> proto_mddb::proto::GetInstrumentByFigiRequest {
    proto_mddb::proto::GetInstrumentByFigiRequest {
        instrument_figi: instrument_figi.to_string(),
    }
}

pub fn get_instrument_by_pair_figi_request(
    instrument_pair_figi: &str,
) -> proto_mddb::proto::GetInstrumentByPairFigiRequest {
    proto_mddb::proto::GetInstrumentByPairFigiRequest {
        instrument_pair_figi: instrument_pair_figi.to_string(),
    }
}

pub fn get_all_instruments_request() -> proto_mddb::proto::GetAllInstrumentsRequest {
    proto_mddb::proto::GetAllInstrumentsRequest {}
}

pub fn get_all_instruments_for_base_asset_request(
    base_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForBaseAssetRequest {
    proto_mddb::proto::GetAllInstrumentsForBaseAssetRequest {
        base_asset: base_asset.to_string(),
    }
}

pub fn get_all_instruments_for_quote_asset_request(
    quote_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForQuoteAssetRequest {
    proto_mddb::proto::GetAllInstrumentsForQuoteAssetRequest {
        quote_asset: quote_asset.to_string(),
    }
}

pub fn get_all_instruments_for_exchange_request(
    exchange_code: &str,
) -> proto_mddb::proto::GetAllInstrumentsForExchangeRequest {
    proto_mddb::proto::GetAllInstrumentsForExchangeRequest {
        exchange_code: exchange_code.to_string(),
    }
}

pub fn get_all_instruments_for_base_asset_and_exchange_request(
    exchange_code: &str,
    base_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForBaseAssetAndExchangeRequest {
    proto_mddb::proto::GetAllInstrumentsForBaseAssetAndExchangeRequest {
        exchange_code: exchange_code.to_string(),
        base_asset: base_asset.to_string(),
    }
}

pub fn get_all_instruments_for_quote_asset_and_exchange_request(
    exchange_code: &str,
    quote_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForQuoteAssetAndExchangeRequest {
    proto_mddb::proto::GetAllInstrumentsForQuoteAssetAndExchangeRequest {
        exchange_code: exchange_code.to_string(),
        quote_asset: quote_asset.to_string(),
    }
}

pub fn get_all_instruments_for_base_quote_asset_and_exchange_request(
    exchange_code: &str,
    base_asset: &str,
    quote_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForBaseQuoteAssetAndExchangeRequest {
    proto_mddb::proto::GetAllInstrumentsForBaseQuoteAssetAndExchangeRequest {
        exchange_code: exchange_code.to_string(),
        base_asset: base_asset.to_string(),
        quote_asset: quote_asset.to_string(),
    }
}

pub fn get_lookup_instrument_exchange_pair_code_request(
    instrument_exchange_pair_code: &str,
) -> proto_mddb::proto::LookupInstrumentIdByExchangePairCodeRequest {
    proto_mddb::proto::LookupInstrumentIdByExchangePairCodeRequest {
        instrument_exchange_pair_code: instrument_exchange_pair_code.to_string(),
    }
}

pub fn get_lookup_instrument_id_by_figi_request(
    instrument_id: &str,
) -> proto_mddb::proto::LookupInstrumentIdByFigiRequest {
    proto_mddb::proto::LookupInstrumentIdByFigiRequest {
        instrument_figi: instrument_id.to_string(),
    }
}

pub fn get_lookup_instrument_id_by_pair_figi_request(
    instrument_pair_figi: &str,
) -> proto_mddb::proto::LookupInstrumentIdByPairFigiRequest {
    proto_mddb::proto::LookupInstrumentIdByPairFigiRequest {
        instrument_pair_figi: instrument_pair_figi.to_string(),
    }
}

// Response

pub fn get_count_instruments_response(count: u64) -> proto_mddb::proto::CountInstrumentsResponse {
    proto_mddb::proto::CountInstrumentsResponse { count }
}

pub fn get_check_if_instrument_exists_response(
    instrument_id: &str,
    exists: bool,
) -> proto_mddb::proto::CheckIfInstrumentIdExistsResponse {
    proto_mddb::proto::CheckIfInstrumentIdExistsResponse {
        instrument_id: instrument_id.to_string(),
        exists,
    }
}

pub fn get_instrument_by_id_response(
    meta_instrument: Option<MetaInstrument>,
) -> proto_mddb::proto::GetInstrumentByIdResponse {
    if let Some(instrument) = meta_instrument {
        proto_mddb::proto::GetInstrumentByIdResponse {
            instrument: Option::from(instruments_utils::meta_instrument_to_proto_instrument(
                &instrument,
            )),
        }
    } else {
        proto_mddb::proto::GetInstrumentByIdResponse { instrument: None }
    }
}

pub fn get_instrument_by_figi_response(
    meta_instrument: Option<MetaInstrument>,
) -> proto_mddb::proto::GetInstrumentByFigiResponse {
    if let Some(instrument) = meta_instrument {
        proto_mddb::proto::GetInstrumentByFigiResponse {
            instrument: Option::from(instruments_utils::meta_instrument_to_proto_instrument(
                &instrument,
            )),
        }
    } else {
        proto_mddb::proto::GetInstrumentByFigiResponse { instrument: None }
    }
}

pub fn get_instrument_by_pair_figi_response(
    meta_instrument: Option<MetaInstrument>,
) -> proto_mddb::proto::GetInstrumentByPairFigiResponse {
    if let Some(instrument) = meta_instrument {
        proto_mddb::proto::GetInstrumentByPairFigiResponse {
            instrument: Option::from(instruments_utils::meta_instrument_to_proto_instrument(
                &instrument,
            )),
        }
    } else {
        proto_mddb::proto::GetInstrumentByPairFigiResponse { instrument: None }
    }
}

pub fn get_all_instruments_response(
    meta_instruments: Vec<MetaInstrument>,
) -> proto_mddb::proto::GetAllInstrumentsResponse {
    proto_mddb::proto::GetAllInstrumentsResponse {
        instruments: meta_instruments
            .into_iter()
            .map(|meta_instrument: MetaInstrument| {
                instruments_utils::meta_instrument_to_proto_instrument(&meta_instrument)
            })
            .collect(),
    }
}

pub fn get_all_instruments_for_base_asset_response(
    meta_instruments: Vec<MetaInstrument>,
) -> proto_mddb::proto::GetAllInstrumentsForBaseAssetResponse {
    proto_mddb::proto::GetAllInstrumentsForBaseAssetResponse {
        instruments: meta_instruments
            .into_iter()
            .map(|meta_instrument: MetaInstrument| {
                instruments_utils::meta_instrument_to_proto_instrument(&meta_instrument)
            })
            .collect(),
    }
}

pub fn get_all_instruments_for_quote_asset_response(
    meta_instruments: Vec<MetaInstrument>,
) -> proto_mddb::proto::GetAllInstrumentsForQuoteAssetResponse {
    proto_mddb::proto::GetAllInstrumentsForQuoteAssetResponse {
        instruments: meta_instruments
            .into_iter()
            .map(|meta_instrument: MetaInstrument| {
                instruments_utils::meta_instrument_to_proto_instrument(&meta_instrument)
            })
            .collect(),
    }
}

pub fn get_all_instruments_for_exchange_response(
    meta_instruments: Vec<MetaInstrument>,
) -> proto_mddb::proto::GetAllInstrumentsForExchangeResponse {
    proto_mddb::proto::GetAllInstrumentsForExchangeResponse {
        instruments: meta_instruments
            .into_iter()
            .map(|meta_instrument: MetaInstrument| {
                instruments_utils::meta_instrument_to_proto_instrument(&meta_instrument)
            })
            .collect(),
    }
}

pub fn get_all_instruments_for_base_asset_and_exchange_response(
    meta_instruments: Vec<MetaInstrument>,
) -> proto_mddb::proto::GetAllInstrumentsForBaseAssetAndExchangeResponse {
    proto_mddb::proto::GetAllInstrumentsForBaseAssetAndExchangeResponse {
        instruments: meta_instruments
            .into_iter()
            .map(|meta_instrument: MetaInstrument| {
                instruments_utils::meta_instrument_to_proto_instrument(&meta_instrument)
            })
            .collect(),
    }
}

pub fn get_all_instruments_for_quote_asset_and_exchange_response(
    meta_instruments: Vec<MetaInstrument>,
) -> proto_mddb::proto::GetAllInstrumentsForQuoteAssetAndExchangeResponse {
    proto_mddb::proto::GetAllInstrumentsForQuoteAssetAndExchangeResponse {
        instruments: meta_instruments
            .into_iter()
            .map(|meta_instrument: MetaInstrument| {
                instruments_utils::meta_instrument_to_proto_instrument(&meta_instrument)
            })
            .collect(),
    }
}

pub fn get_all_instruments_for_base_quote_asset_and_exchange_response(
    meta_instruments: Vec<MetaInstrument>,
) -> proto_mddb::proto::GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse {
    proto_mddb::proto::GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse {
        instruments: meta_instruments
            .into_iter()
            .map(|meta_instrument: MetaInstrument| {
                instruments_utils::meta_instrument_to_proto_instrument(&meta_instrument)
            })
            .collect(),
    }
}

pub fn get_lookup_instrument_id_by_exchange_pair_code_response(
    para_instrument: Option<MetaInstrument>,
) -> proto_mddb::proto::LookupInstrumentIdByExchangePairCodeResponse {
    if let Some(instrument) = para_instrument {
        proto_mddb::proto::LookupInstrumentIdByExchangePairCodeResponse {
            instrument_id: Option::from(instrument.primary_key()),
        }
    } else {
        proto_mddb::proto::LookupInstrumentIdByExchangePairCodeResponse {
            instrument_id: None,
        }
    }
}

pub fn get_lookup_instrument_by_figi_response(
    para_instrument: Option<MetaInstrument>,
) -> proto_mddb::proto::LookupInstrumentIdByFigiResponse {
    if let Some(instrument) = para_instrument {
        proto_mddb::proto::LookupInstrumentIdByFigiResponse {
            instrument_id: Option::from(instrument.primary_key()),
        }
    } else {
        proto_mddb::proto::LookupInstrumentIdByFigiResponse {
            instrument_id: None,
        }
    }
}

pub fn get_lookup_instrument_by_pair_figi_response(
    para_instrument: Option<MetaInstrument>,
) -> proto_mddb::proto::LookupInstrumentIdByPairFigiResponse {
    if let Some(instrument) = para_instrument {
        proto_mddb::proto::LookupInstrumentIdByPairFigiResponse {
            instrument_id: Option::from(instrument.primary_key()),
        }
    } else {
        proto_mddb::proto::LookupInstrumentIdByPairFigiResponse {
            instrument_id: None,
        }
    }
}
