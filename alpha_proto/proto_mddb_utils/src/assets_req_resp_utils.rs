/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::assets_utils::meta_asset_to_proto_asset;
use common_metadata::MetaAsset;
use proto_mddb::proto::{
    CheckIfAssetIdExistsRequest, CheckIfAssetIdExistsResponse, CountAssetsRequest,
    CountAssetsResponse, GetAllAssetsRequest, GetAllAssetsResponse, GetAssetRequest,
    GetAssetResponse,
};

// Asset Request
/// Creates a new request to count all assets in the database.
///
/// # Returns
/// Returns an empty [`CountAssetsRequest`] struct that can be used to query the total number of assets.
///
/// # Implementation Notes
/// - This is a const function that creates an empty request with no parameters
/// - Used as part of the asset counting workflow in the metadata database
#[must_use]
pub const fn get_count_assets_request() -> CountAssetsRequest {
    CountAssetsRequest {}
}

/// Creates a new request to check if an asset with the given ID exists.
///
/// # Arguments
/// * `asset_id` - The unique identifier of the asset to check
///
/// # Returns
/// Returns a [`CheckIfAssetIdExistsRequest`] containing the provided asset ID.
///
/// # Implementation Notes
/// - Clones the input string to create an owned version for the request
/// - Used for asset existence validation before performing operations that require the asset to exist
#[must_use]
pub fn get_check_if_asset_exists_request(asset_id: &str) -> CheckIfAssetIdExistsRequest {
    CheckIfAssetIdExistsRequest {
        asset_id: asset_id.to_string(),
    }
}

/// Creates a new request to retrieve an asset by its ID.
///
/// # Arguments
/// * `asset_id` - The unique identifier of the asset to retrieve
///
/// # Returns
/// Returns a [`GetAssetRequest`] containing the provided asset ID.
///
/// # Implementation Notes
/// - Clones the input string to create an owned version for the request
/// - Used to fetch detailed information about a specific asset
#[must_use]
pub fn get_asset_request(asset_id: &str) -> GetAssetRequest {
    GetAssetRequest {
        asset_id: asset_id.to_string(),
    }
}

/// Creates a new request to retrieve all assets from the database.
///
/// # Returns
/// Returns an empty [`GetAllAssetsRequest`] struct that can be used to query all assets.
///
/// # Implementation Notes
/// - This is a const function that creates an empty request with no parameters
/// - Used when a complete list of all assets is needed
/// - Consider pagination for large datasets to avoid memory issues
#[must_use]
pub const fn get_all_assets_request() -> GetAllAssetsRequest {
    GetAllAssetsRequest {}
}

// Asset Response
/// Creates a response containing the total count of assets.
///
/// # Arguments
/// * `count` - The total number of assets in the database
///
/// # Returns
/// Returns a [`CountAssetsResponse`] containing the provided count.
///
/// # Implementation Notes
/// - This is a const function that wraps the count in a response struct
/// - Used as part of the asset counting workflow
#[must_use]
pub const fn get_count_assets_response(count: u64) -> CountAssetsResponse {
    CountAssetsResponse { count }
}

/// Creates a response indicating whether an asset exists.
///
/// # Arguments
/// * `exists` - Boolean indicating if the asset exists
///
/// # Returns
/// Returns a [`CheckIfAssetIdExistsResponse`] containing the existence status.
///
/// # Implementation Notes
/// - This is a const function that wraps the boolean in a response struct
/// - Used for asset existence validation responses
#[must_use]
pub const fn get_check_if_asset_exists_response(exists: bool) -> CheckIfAssetIdExistsResponse {
    CheckIfAssetIdExistsResponse { exists }
}

/// Creates a response containing an optional asset.
///
/// # Arguments
/// * `meta_asset` - Optional [`MetaAsset`] to include in the response
///
/// # Returns
/// Returns a [`GetAssetResponse`] containing the asset if provided, or None if not.
///
/// # Implementation Notes
/// - Converts the `MetaAsset` to a `ProtoMetaAsset` if present
/// - Returns an empty response if None is provided
/// - Used for single asset retrieval responses
#[must_use]
pub fn get_assets_response(meta_asset: Option<MetaAsset>) -> GetAssetResponse {
    if let Some(assets) = meta_asset {
        GetAssetResponse {
            asset: Option::from(meta_asset_to_proto_asset(&assets)),
        }
    } else {
        GetAssetResponse { asset: None }
    }
}

/// Creates a response containing a list of assets.
///
/// # Arguments
/// * `assets` - Slice of [`MetaAsset`]s to include in the response
///
/// # Returns
/// Returns a [`GetAllAssetsResponse`] containing all provided assets converted to proto format.
///
/// # Implementation Notes
/// - Efficiently converts each `MetaAsset` to `ProtoMetaAsset` using iterators
/// - Takes a reference to avoid unnecessary cloning of the input data
/// - Used for bulk asset retrieval responses
#[must_use]
pub fn get_all_assets_response(assets: &[MetaAsset]) -> GetAllAssetsResponse {
    GetAllAssetsResponse {
        assets: assets
            .iter()
            .map(|meta_asset: &MetaAsset| meta_asset_to_proto_asset(meta_asset))
            .collect::<Vec<proto_mddb::proto::ProtoMetaAsset>>(),
    }
}
