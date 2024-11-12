use common_metadata::prelude::{InstrumentMetadata, MetaInstrument};

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
