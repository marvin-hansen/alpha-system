/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::model::instrument::Instrument;
use crate::{CreateInstrument, UpdateInstrument};
use common_metadata::{InstrumentMetadata, MetaInstrument};

impl Instrument {
    #[must_use]
    pub fn from_meta_instrument(meta_instrument: MetaInstrument) -> Self {
        Self {
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
            instrument_trade_start_timestamp: meta_instrument
                .trade_start_timestamp
                .map(|ts| ts as i64),
            instrument_trade_end_timestamp: meta_instrument.trade_end_timestamp,
        }
    }

    #[must_use]
    pub fn to_meta_instrument(&self) -> MetaInstrument {
        let metadata = if self.instrument_figi.is_some() || self.instrument_pair_figi.is_some() {
            Some(InstrumentMetadata {
                pair_figi: self.instrument_pair_figi.clone(),
                instrument_figi: self.instrument_figi.clone(),
            })
        } else {
            None
        };

        MetaInstrument {
            kaiko_legacy_exchange_slug: String::new(), //  kaiko_legacy_exchange_slug is not used
            trade_start_time: Some(self.instrument_trade_start_timestamp.unwrap().to_string()),
            trade_end_time: Some(self.instrument_trade_start_timestamp.unwrap().to_string()),
            exchange_code: self.instrument_exchanges_code.clone(),
            exchange_pair_code: self.instrument_exchange_pair_code.clone(),
            base_asset: self.instrument_base_asset.clone(),
            quote_asset: self.instrument_quote_asset.clone(),
            kaiko_legacy_symbol: String::new(), //  kaiko_legacy_symbol is not used
            code: self.instrument_code.clone(),
            class: self.instrument_class.clone(),
            metadata,
            trade_start_timestamp: self.instrument_trade_start_timestamp.map(|ts| ts as u64),
            trade_end_timestamp: self.instrument_trade_end_timestamp,
            trade_compressed_size: 0, //  trade_compressed_size is not used
            trade_count: 0,           //  trade_count is not used
        }
    }
}

impl CreateInstrument {
    #[must_use]
    pub fn from_meta_instrument(meta_instrument: MetaInstrument) -> Self {
        Self {
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
            instrument_trade_start_timestamp: meta_instrument
                .trade_start_timestamp
                .map(|ts| ts as i64),
            instrument_trade_end_timestamp: meta_instrument.trade_end_timestamp,
        }
    }
}

impl UpdateInstrument {
    #[must_use]
    pub fn from_meta_instrument(meta_instrument: MetaInstrument) -> Self {
        Self {
            instrument_class: meta_instrument.class.to_string(),
            instrument_code: meta_instrument.code.to_string(),
            instrument_hash: meta_instrument.hash(),
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
            instrument_trade_start_timestamp: meta_instrument
                .trade_start_timestamp
                .map(|ts| ts as i64),
            instrument_trade_end_timestamp: meta_instrument.trade_end_timestamp,
        }
    }
}
