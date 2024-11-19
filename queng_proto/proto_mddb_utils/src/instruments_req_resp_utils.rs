use crate::instruments_utils;
use common_metadata::MetaInstrument;

/// Creates a request to count all instruments in the database.
///
/// # Returns
///
/// Returns an empty `CountInstrumentsRequest` message.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub const fn get_count_instruments_request() -> proto_mddb::proto::CountInstrumentsRequest {
    proto_mddb::proto::CountInstrumentsRequest {}
}

/// Creates a request to check if an instrument exists by its ID.
///
/// # Arguments
///
/// * `instrument_id` - The ID of the instrument to check.
///
/// # Returns
///
/// Returns a `CheckIfInstrumentIdExistsRequest` containing the instrument ID.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_check_if_instrument_exists_request(
    instrument_id: &str,
) -> proto_mddb::proto::CheckIfInstrumentIdExistsRequest {
    proto_mddb::proto::CheckIfInstrumentIdExistsRequest {
        instrument_id: instrument_id.to_string(),
    }
}

/// Creates a request to retrieve an instrument by its ID.
///
/// # Arguments
///
/// * `instrument_id` - The ID of the instrument to retrieve.
///
/// # Returns
///
/// Returns a `GetInstrumentByIdRequest` containing the instrument ID.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_instrument_by_id_request(
    instrument_id: &str,
) -> proto_mddb::proto::GetInstrumentByIdRequest {
    proto_mddb::proto::GetInstrumentByIdRequest {
        instrument_id: instrument_id.to_string(),
    }
}

/// Creates a request to retrieve an instrument by its FIGI.
///
/// # Arguments
///
/// * `instrument_figi` - The FIGI of the instrument to retrieve.
///
/// # Returns
///
/// Returns a `GetInstrumentByFigiRequest` containing the instrument FIGI.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_instrument_by_figi_request(
    instrument_figi: &str,
) -> proto_mddb::proto::GetInstrumentByFigiRequest {
    proto_mddb::proto::GetInstrumentByFigiRequest {
        instrument_figi: instrument_figi.to_string(),
    }
}

/// Creates a request to retrieve an instrument by its pair FIGI.
///
/// # Arguments
///
/// * `instrument_pair_figi` - The pair FIGI of the instrument to retrieve.
///
/// # Returns
///
/// Returns a `GetInstrumentByPairFigiRequest` containing the instrument pair FIGI.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_instrument_by_pair_figi_request(
    instrument_pair_figi: &str,
) -> proto_mddb::proto::GetInstrumentByPairFigiRequest {
    proto_mddb::proto::GetInstrumentByPairFigiRequest {
        instrument_pair_figi: instrument_pair_figi.to_string(),
    }
}

/// Creates a request to retrieve all instruments.
///
/// # Returns
///
/// Returns an empty `GetAllInstrumentsRequest` message.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub const fn get_all_instruments_request() -> proto_mddb::proto::GetAllInstrumentsRequest {
    proto_mddb::proto::GetAllInstrumentsRequest {}
}

/// Creates a request to retrieve all instruments for a given base asset.
///
/// # Arguments
///
/// * `base_asset` - The base asset code to filter instruments by.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForBaseAssetRequest` containing the base asset code.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_all_instruments_for_base_asset_request(
    base_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForBaseAssetRequest {
    proto_mddb::proto::GetAllInstrumentsForBaseAssetRequest {
        base_asset: base_asset.to_string(),
    }
}

/// Creates a request to retrieve all instruments for a given quote asset.
///
/// # Arguments
///
/// * `quote_asset` - The quote asset code to filter instruments by.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForQuoteAssetRequest` containing the quote asset code.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_all_instruments_for_quote_asset_request(
    quote_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForQuoteAssetRequest {
    proto_mddb::proto::GetAllInstrumentsForQuoteAssetRequest {
        quote_asset: quote_asset.to_string(),
    }
}

/// Creates a request to retrieve all instruments for a given exchange.
///
/// # Arguments
///
/// * `exchange_code` - The exchange code to filter instruments by.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForExchangeRequest` containing the exchange code.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_all_instruments_for_exchange_request(
    exchange_code: &str,
) -> proto_mddb::proto::GetAllInstrumentsForExchangeRequest {
    proto_mddb::proto::GetAllInstrumentsForExchangeRequest {
        exchange_code: exchange_code.to_string(),
    }
}

/// Creates a request to retrieve all instruments for a given base asset and exchange.
///
/// # Arguments
///
/// * `exchange_code` - The exchange code to filter instruments by.
/// * `base_asset` - The base asset code to filter instruments by.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForBaseAssetAndExchangeRequest` containing the exchange code and base asset code.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input strings to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_all_instruments_for_base_asset_and_exchange_request(
    exchange_code: &str,
    base_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForBaseAssetAndExchangeRequest {
    proto_mddb::proto::GetAllInstrumentsForBaseAssetAndExchangeRequest {
        exchange_code: exchange_code.to_string(),
        base_asset: base_asset.to_string(),
    }
}

/// Creates a request to retrieve all instruments for a given quote asset and exchange.
///
/// # Arguments
///
/// * `exchange_code` - The exchange code to filter instruments by.
/// * `quote_asset` - The quote asset code to filter instruments by.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForQuoteAssetAndExchangeRequest` containing the exchange code and quote asset code.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input strings to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_all_instruments_for_quote_asset_and_exchange_request(
    exchange_code: &str,
    quote_asset: &str,
) -> proto_mddb::proto::GetAllInstrumentsForQuoteAssetAndExchangeRequest {
    proto_mddb::proto::GetAllInstrumentsForQuoteAssetAndExchangeRequest {
        exchange_code: exchange_code.to_string(),
        quote_asset: quote_asset.to_string(),
    }
}

/// Creates a request to retrieve all instruments for a given base asset, quote asset, and exchange.
///
/// # Arguments
///
/// * `exchange_code` - The exchange code to filter instruments by.
/// * `base_asset` - The base asset code to filter instruments by.
/// * `quote_asset` - The quote asset code to filter instruments by.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForBaseQuoteAssetAndExchangeRequest` containing the exchange code, base asset code, and quote asset code.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input strings to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
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

/// Creates a request to look up an instrument by its exchange pair code.
///
/// # Arguments
///
/// * `instrument_exchange_pair_code` - The exchange pair code to look up the instrument by.
///
/// # Returns
///
/// Returns a `LookupInstrumentIdByExchangePairCodeRequest` containing the instrument exchange pair code.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_lookup_instrument_exchange_pair_code_request(
    instrument_exchange_pair_code: &str,
) -> proto_mddb::proto::LookupInstrumentIdByExchangePairCodeRequest {
    proto_mddb::proto::LookupInstrumentIdByExchangePairCodeRequest {
        instrument_exchange_pair_code: instrument_exchange_pair_code.to_string(),
    }
}

/// Creates a request to look up an instrument by its FIGI.
///
/// # Arguments
///
/// * `instrument_id` - The FIGI to look up the instrument by.
///
/// # Returns
///
/// Returns a `LookupInstrumentIdByFigiRequest` containing the instrument FIGI.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_lookup_instrument_id_by_figi_request(
    instrument_id: &str,
) -> proto_mddb::proto::LookupInstrumentIdByFigiRequest {
    proto_mddb::proto::LookupInstrumentIdByFigiRequest {
        instrument_figi: instrument_id.to_string(),
    }
}

/// Creates a request to look up an instrument by its pair FIGI.
///
/// # Arguments
///
/// * `instrument_pair_figi` - The pair FIGI to look up the instrument by.
///
/// # Returns
///
/// Returns a `LookupInstrumentIdByPairFigiRequest` containing the instrument pair FIGI.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned request.
///
#[must_use]
pub fn get_lookup_instrument_id_by_pair_figi_request(
    instrument_pair_figi: &str,
) -> proto_mddb::proto::LookupInstrumentIdByPairFigiRequest {
    proto_mddb::proto::LookupInstrumentIdByPairFigiRequest {
        instrument_pair_figi: instrument_pair_figi.to_string(),
    }
}

// Response

/// Creates a response with the count of instruments.
///
/// # Arguments
///
/// * `count` - The count of instruments.
///
/// # Returns
///
/// Returns a `CountInstrumentsResponse` containing the count.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
pub const fn get_count_instruments_response(
    count: u64,
) -> proto_mddb::proto::CountInstrumentsResponse {
    proto_mddb::proto::CountInstrumentsResponse { count }
}

/// Creates a response indicating whether an instrument exists by its ID.
///
/// # Arguments
///
/// * `instrument_id` - The ID of the instrument.
/// * `exists` - Whether the instrument exists.
///
/// # Returns
///
/// Returns a `CheckIfInstrumentIdExistsResponse` containing the instrument ID and existence.
///
/// # Implementation Notes
///
/// This function performs a deep copy of the input string to ensure proper ownership.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
pub fn get_check_if_instrument_exists_response(
    instrument_id: &str,
    exists: bool,
) -> proto_mddb::proto::CheckIfInstrumentIdExistsResponse {
    proto_mddb::proto::CheckIfInstrumentIdExistsResponse {
        instrument_id: instrument_id.to_string(),
        exists,
    }
}

/// Creates a response with an instrument retrieved by its ID.
///
/// # Arguments
///
/// * `meta_instrument` - The instrument retrieved by its ID.
///
/// # Returns
///
/// Returns a `GetInstrumentByIdResponse` containing the instrument.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument` to a `proto_mddb::proto::Instrument` using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with an instrument retrieved by its FIGI.
///
/// # Arguments
///
/// * `meta_instrument` - The instrument retrieved by its FIGI.
///
/// # Returns
///
/// Returns a `GetInstrumentByFigiResponse` containing the instrument.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument` to a `proto_mddb::proto::Instrument` using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with an instrument retrieved by its pair FIGI.
///
/// # Arguments
///
/// * `meta_instrument` - The instrument retrieved by its pair FIGI.
///
/// # Returns
///
/// Returns a `GetInstrumentByPairFigiResponse` containing the instrument.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument` to a `proto_mddb::proto::Instrument` using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with all instruments.
///
/// # Arguments
///
/// * `meta_instruments` - The instruments to include in the response.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsResponse` containing the instruments.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument`s to `proto_mddb::proto::Instrument`s using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with all instruments for a given base asset.
///
/// # Arguments
///
/// * `meta_instruments` - The instruments to include in the response.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForBaseAssetResponse` containing the instruments.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument`s to `proto_mddb::proto::Instrument`s using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with all instruments for a given quote asset.
///
/// # Arguments
///
/// * `meta_instruments` - The instruments to include in the response.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForQuoteAssetResponse` containing the instruments.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument`s to `proto_mddb::proto::Instrument`s using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with all instruments for a given exchange.
///
/// # Arguments
///
/// * `meta_instruments` - The instruments to include in the response.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForExchangeResponse` containing the instruments.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument`s to `proto_mddb::proto::Instrument`s using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with all instruments for a given base asset and exchange.
///
/// # Arguments
///
/// * `meta_instruments` - The instruments to include in the response.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForBaseAssetAndExchangeResponse` containing the instruments.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument`s to `proto_mddb::proto::Instrument`s using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with all instruments for a given quote asset and exchange.
///
/// # Arguments
///
/// * `meta_instruments` - The instruments to include in the response.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForQuoteAssetAndExchangeResponse` containing the instruments.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument`s to `proto_mddb::proto::Instrument`s using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with all instruments for a given base asset, quote asset, and exchange.
///
/// # Arguments
///
/// * `meta_instruments` - The instruments to include in the response.
///
/// # Returns
///
/// Returns a `GetAllInstrumentsForBaseQuoteAssetAndExchangeResponse` containing the instruments.
///
/// # Implementation Notes
///
/// This function converts the `MetaInstrument`s to `proto_mddb::proto::Instrument`s using the `instruments_utils::meta_instrument_to_proto_instrument` function.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with the instrument ID looked up by its exchange pair code.
///
/// # Arguments
///
/// * `para_instrument` - The instrument looked up by its exchange pair code.
///
/// # Returns
///
/// Returns a `LookupInstrumentIdByExchangePairCodeResponse` containing the instrument ID.
///
/// # Implementation Notes
///
/// This function extracts the primary key from the `MetaInstrument` using the `primary_key` method.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with the instrument ID looked up by its FIGI.
///
/// # Arguments
///
/// * `para_instrument` - The instrument looked up by its FIGI.
///
/// # Returns
///
/// Returns a `LookupInstrumentIdByFigiResponse` containing the instrument ID.
///
/// # Implementation Notes
///
/// This function extracts the primary key from the `MetaInstrument` using the `primary_key` method.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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

/// Creates a response with the instrument ID looked up by its pair FIGI.
///
/// # Arguments
///
/// * `para_instrument` - The instrument looked up by its pair FIGI.
///
/// # Returns
///
/// Returns a `LookupInstrumentIdByPairFigiResponse` containing the instrument ID.
///
/// # Implementation Notes
///
/// This function extracts the primary key from the `MetaInstrument` using the `primary_key` method.
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned response.
///
#[must_use]
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
