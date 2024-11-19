use common_metadata::{InstrumentMetadata, MetaInstrument};

/// Converts a `MetaInstrument` to its protobuf representation.
///
/// # Arguments
///
/// * `meta_instrument` - The `MetaInstrument` to convert.
///
/// # Returns
///
/// Returns a `ProtoMetaInstrument` containing all the converted fields from the input.
///
/// # Implementation Notes
///
/// This function:
/// 1. Maps all basic fields directly (id, code, hash, class, assets)
/// 2. Handles optional FIGI fields by:
///    - Safely unwrapping metadata if present
///    - Providing None for missing FIGI values
/// 3. Converts timestamp fields:
///    - Maps `trade_start_timestamp` from u64 to i64
///    - Preserves `trade_end_timestamp` as is
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned value.
/// All string conversions are performed using `clone()` to ensure ownership transfer.
///
#[must_use]
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

/// Converts a protobuf `ProtoMetaInstrument` back to a `MetaInstrument`.
///
/// # Arguments
///
/// * `proto_instrument` - The `ProtoMetaInstrument` to convert back.
///
/// # Returns
///
/// Returns a `MetaInstrument` containing all the converted fields from the input.
///
/// # Implementation Notes
///
/// This function:
/// 1. Constructs optional metadata if either FIGI field is present
/// 2. Initializes legacy fields with empty strings (`kaiko_legacy_exchange_slug`, `kaiko_legacy_symbol`)
/// 3. Sets unused numeric fields to 0 (`trade_compressed_size`, `trade_count`)
/// 4. Converts timestamps:
///    - Maps `trade_start_timestamp` from i64 to u64
///    - Preserves `trade_end_timestamp` as is
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned value.
/// All string conversions are performed using `clone()` to ensure ownership transfer.
///
#[must_use]
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
