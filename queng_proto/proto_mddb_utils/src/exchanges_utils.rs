/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_metadata::MetaExchange;

/// Converts a `MetaExchange` to its protobuf representation.
///
/// # Arguments
///
/// * `meta_exchange` - The `MetaExchange` to convert.
///
/// # Returns
///
/// Returns a `ProtoMetaExchange` containing all the converted fields from the input.
///
/// # Implementation Notes
///
/// This function:
/// 1. Maps all basic fields (code, name) using string conversions
/// 2. Computes and includes the exchange hash
/// 3. Performs deep copies of strings to ensure ownership transfer
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned value.
/// All string conversions are performed using `to_string()` to ensure proper ownership.
///
#[must_use]
pub fn meta_exchange_to_proto_exchange(
    meta_exchange: &MetaExchange,
) -> proto_mddb::proto::ProtoMetaExchange {
    proto_mddb::proto::ProtoMetaExchange {
        exchange_code: meta_exchange.code.to_string(),
        exchange_name: meta_exchange.name.to_string(),
        exchange_hash: meta_exchange.hash(),
    }
}

/// Converts a protobuf `ProtoMetaExchange` back to a `MetaExchange`.
///
/// # Arguments
///
/// * `proto_exchange` - The `ProtoMetaExchange` to convert back.
///
/// # Returns
///
/// Returns a `MetaExchange` containing all the converted fields from the input.
///
/// # Implementation Notes
///
/// This function:
/// 1. Maps basic fields (code, name) using string conversions
/// 2. Initializes legacy field (`kaiko_legacy_slug`) with an empty string
/// 3. Performs deep copies of strings to ensure ownership transfer
///
/// # Safety
///
/// This function is marked as `#[must_use]` to ensure the caller handles the returned value.
/// All string conversions are performed using `to_string()` to ensure proper ownership.
///
#[must_use]
pub fn proto_exchange_to_meta_exchange(
    proto_exchange: &proto_mddb::proto::ProtoMetaExchange,
) -> MetaExchange {
    MetaExchange {
        code: proto_exchange.exchange_code.to_string(),
        name: proto_exchange.exchange_name.to_string(),
        kaiko_legacy_slug: String::new(),
    }
}
