/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::exchanges_utils;
use common_metadata::MetaExchange;
use proto_mddb::proto::{
    CheckIfExchangeIdExistsRequest, CheckIfExchangeIdExistsResponse, CountExchangesRequest,
    CountExchangesResponse, GetAllExchangesRequest, GetExchangeRequest, LookupExchangeNameRequest,
};

// Request
/// Creates a new request to count all exchanges in the database.
///
/// # Returns
/// Returns an empty [`CountExchangesRequest`] struct that can be used to query the total number of exchanges.
///
/// # Implementation Notes
/// - This is a const function that creates an empty request with no parameters
/// - Used as part of the exchange counting workflow in the metadata database
/// - The response will contain the total count of all exchanges, regardless of their status
#[must_use]
pub const fn get_count_exchanges_request() -> CountExchangesRequest {
    CountExchangesRequest {}
}

/// Creates a new request to check if an exchange with the given code exists.
///
/// # Arguments
/// * `exchange_code` - The unique identifier code of the exchange to check
///
/// # Returns
/// Returns a [`CheckIfExchangeIdExistsRequest`] containing the provided exchange code.
///
/// # Implementation Notes
/// - Clones the input string to create an owned version for the request
/// - Used for exchange existence validation before performing operations that require the exchange to exist
/// - The exchange code should be a standardized identifier (e.g., "BINANCE", "KRAKEN")
#[must_use]
pub fn get_check_if_exchange_exists_request(exchange_code: &str) -> CheckIfExchangeIdExistsRequest {
    CheckIfExchangeIdExistsRequest {
        exchange_code: exchange_code.to_string(),
    }
}

/// Creates a new request to retrieve an exchange by its code.
///
/// # Arguments
/// * `exchange_code` - The unique identifier code of the exchange to retrieve
///
/// # Returns
/// Returns a [`GetExchangeRequest`] containing the provided exchange code.
///
/// # Implementation Notes
/// - Clones the input string to create an owned version for the request
/// - Used to fetch detailed information about a specific exchange
/// - The exchange code should be a standardized identifier (e.g., "BINANCE", "KRAKEN")
#[must_use]
pub fn get_exchange_request(exchange_code: &str) -> GetExchangeRequest {
    GetExchangeRequest {
        exchange_code: exchange_code.to_string(),
    }
}

/// Creates a new request to retrieve all exchanges from the database.
///
/// # Returns
/// Returns an empty [`GetAllExchangesRequest`] struct that can be used to query all exchanges.
///
/// # Implementation Notes
/// - This is a const function that creates an empty request with no parameters
/// - Used when a complete list of all exchanges is needed
/// - Consider memory usage when dealing with a large number of exchanges
#[must_use]
pub const fn get_all_exchanges_request() -> GetAllExchangesRequest {
    GetAllExchangesRequest {}
}

/// Creates a new request to look up an exchange's name by its code.
///
/// # Arguments
/// * `exchange_code` - The unique identifier code of the exchange to look up
///
/// # Returns
/// Returns a [`LookupExchangeNameRequest`] containing the provided exchange code.
///
/// # Implementation Notes
/// - Clones the input string to create an owned version for the request
/// - Used for quick name lookups without fetching the entire exchange details
/// - The exchange code should be a standardized identifier (e.g., "BINANCE", "KRAKEN")
#[must_use]
pub fn get_lookup_exchange_name_request(exchange_code: &str) -> LookupExchangeNameRequest {
    LookupExchangeNameRequest {
        exchange_code: exchange_code.to_string(),
    }
}

// Response

/// Creates a response containing the total count of exchanges.
///
/// # Arguments
/// * `count` - The total number of exchanges in the database
///
/// # Returns
/// Returns a [`CountExchangesResponse`] containing the provided count.
///
/// # Implementation Notes
/// - This is a const function that wraps the count in a response struct
/// - Used as part of the exchange counting workflow
/// - The count represents all exchanges in the database, regardless of their status
#[must_use]
pub const fn get_count_exchanges_response(count: u64) -> CountExchangesResponse {
    CountExchangesResponse { count }
}

/// Creates a response indicating whether an exchange exists.
///
/// # Arguments
/// * `exists` - Boolean indicating if the exchange exists
///
/// # Returns
/// Returns a [`CheckIfExchangeIdExistsResponse`] containing the existence status.
///
/// # Implementation Notes
/// - This is a const function that wraps the boolean in a response struct
/// - Used for exchange existence validation responses
/// - A true value indicates the exchange exists in the database
#[must_use]
pub const fn get_check_if_exchange_exists_response(
    exists: bool,
) -> CheckIfExchangeIdExistsResponse {
    CheckIfExchangeIdExistsResponse { exists }
}

/// Creates a response containing an optional exchange.
///
/// # Arguments
/// * `meta_exchange` - Optional [`MetaExchange`] to include in the response
///
/// # Returns
/// Returns a proto::GetExchangeResponse containing the exchange if provided, or None if not.
///
/// # Implementation Notes
/// - Converts the `MetaExchange` to a `ProtoExchange` if present
/// - Returns an empty response if None is provided
/// - Used for single exchange retrieval responses
/// - The conversion preserves all exchange metadata and properties
#[must_use]
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

/// Creates a response containing a list of exchanges.
///
/// # Arguments
/// * `meta_exchanges` - Vector of [`MetaExchange`]s to include in the response
///
/// # Returns
/// Returns a proto::GetAllExchangesResponse containing all provided exchanges converted to proto format.
///
/// # Implementation Notes
/// - Efficiently converts each `MetaExchange` to `ProtoExchange` using `into_iter`
/// - Takes ownership of the input vector for efficiency
/// - Used for bulk exchange retrieval responses
/// - The conversion preserves all exchange metadata and properties
#[must_use]
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

/// Creates a response containing an exchange's name lookup result.
///
/// # Arguments
/// * `exchange` - Optional [`MetaExchange`] containing the exchange information
///
/// # Returns
/// Returns a proto::LookupExchangeNameResponse containing the exchange name if found, or None if not.
///
/// # Implementation Notes
/// - Extracts only the name field from the `MetaExchange` if present
/// - Returns None if the exchange is not found
/// - Used for efficient name lookups without transferring full exchange details
/// - The name is cloned from the exchange data
#[must_use]
pub fn get_lookup_exchange_name_response(
    exchange: Option<MetaExchange>,
) -> proto_mddb::proto::LookupExchangeNameResponse {
    if let Some(exchange) = exchange {
        proto_mddb::proto::LookupExchangeNameResponse {
            exchange_name: Some(exchange.name),
        }
    } else {
        proto_mddb::proto::LookupExchangeNameResponse {
            exchange_name: None,
        }
    }
}
