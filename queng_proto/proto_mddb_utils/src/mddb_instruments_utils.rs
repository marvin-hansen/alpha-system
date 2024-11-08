use common_metadata::prelude::{InstrumentMetadata, MetaInstrument};

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
            instrument: Option::from(meta_instrument_to_proto_instrument(&instrument)),
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
            instrument: Option::from(meta_instrument_to_proto_instrument(&instrument)),
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
            instrument: Option::from(meta_instrument_to_proto_instrument(&instrument)),
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
                meta_instrument_to_proto_instrument(&meta_instrument)
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
                meta_instrument_to_proto_instrument(&meta_instrument)
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
                meta_instrument_to_proto_instrument(&meta_instrument)
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
                meta_instrument_to_proto_instrument(&meta_instrument)
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
                meta_instrument_to_proto_instrument(&meta_instrument)
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
                meta_instrument_to_proto_instrument(&meta_instrument)
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
                meta_instrument_to_proto_instrument(&meta_instrument)
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

// Conversion utils
pub fn meta_instrument_to_proto_instrument(
    meta_instrument: &MetaInstrument,
) -> proto_mddb::proto::ProtoMetaInstrument {
    proto_mddb::proto::ProtoMetaInstrument {
        instrument_id: meta_instrument.primary_key(),
        instrument_code: meta_instrument.code.to_string(),
        instrument_hash: meta_instrument.hash(),
        instrument_class: meta_instrument.class.to_string(),
        instrument_base_asset: meta_instrument.base_asset.to_string(),
        instrument_quote_asset: meta_instrument.quote_asset.to_string(),
        instrument_exchanges_code: meta_instrument.exchange_code.to_string(),
        instrument_exchange_pair_code: meta_instrument.exchange_pair_code.to_string(),
        instrument_pair_figi: if meta_instrument.metadata.is_some() {
            Some(
                meta_instrument
                    .clone()
                    .metadata
                    .unwrap()
                    .pair_figi
                    .unwrap_or_default(),
            )
        } else {
            None
        },
        instrument_figi: if meta_instrument.metadata.is_some() {
            Some(
                meta_instrument
                    .clone()
                    .metadata
                    .unwrap()
                    .instrument_figi
                    .unwrap_or_default(),
            )
        } else {
            None
        },
        instrument_trade_start_timestamp: meta_instrument.trade_start_timestamp.map(|ts| ts as i64),
        instrument_trade_end_timestamp: meta_instrument.trade_end_timestamp,
    }
}

pub fn proto_instrument_to_meta_instrument(
    proto_instrument: &proto_mddb::proto::ProtoMetaInstrument,
) -> MetaInstrument {
    let metadata = if proto_instrument.instrument_figi.is_some()
        || proto_instrument.instrument_pair_figi.is_some()
    {
        Some(InstrumentMetadata {
            pair_figi: proto_instrument.instrument_pair_figi.clone(),
            instrument_figi: proto_instrument.instrument_figi.clone(),
        })
    } else {
        None
    };

    MetaInstrument {
        kaiko_legacy_exchange_slug: String::new(), //  kaiko_legacy_exchange_slug is not used
        trade_start_time: Some(
            proto_instrument
                .instrument_trade_start_timestamp
                .unwrap()
                .to_string(),
        ),
        trade_end_time: Some(
            proto_instrument
                .instrument_trade_start_timestamp
                .unwrap()
                .to_string(),
        ),
        exchange_code: proto_instrument.instrument_exchanges_code.clone(),
        exchange_pair_code: proto_instrument.instrument_exchange_pair_code.clone(),
        base_asset: proto_instrument.instrument_base_asset.clone(),
        quote_asset: proto_instrument.instrument_quote_asset.clone(),
        kaiko_legacy_symbol: String::new(), //  kaiko_legacy_symbol is not used
        code: proto_instrument.instrument_code.clone(),
        class: proto_instrument.instrument_class.clone(),
        metadata,
        trade_start_timestamp: proto_instrument
            .instrument_trade_start_timestamp
            .map(|ts| ts as u64),
        trade_end_timestamp: proto_instrument.instrument_trade_end_timestamp,
        trade_compressed_size: 0, //  trade_compressed_size is not used
        trade_count: 0,           //  trade_count is not used
    }
}
