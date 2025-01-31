/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::print_utils;
use crate::types::worflow_op::{MetaDataDBWOp, WorkflowOp};
use crate::workflow;
use common_metadata::MetaDataSet;
use pg_mddb_manager::PostgresMDDBManager;

/// Update partial metadata in the database.
///
/// This function will update only the specified metadata in the database
/// using the given `PostgresMDDBManager`.
///
/// The `meta_data_ops` parameter specifies which assets, exchanges and
/// instruments should be updated.
///
/// `dbm_mddb` is the `PostgresMDDBManager` to use for creating the database
/// schema.
///
/// `meta_data` is the `MetaDataSet` containing the metadata to be imported.
///
pub async fn update_partial_metadata(
    meta_data_ops: &MetaDataDBWOp,
    dbm_mddb: &PostgresMDDBManager,
    meta_data: &MetaDataSet,
) {
    print_utils::dbg_print("update_partial_metadata");

    if meta_data_ops.assets_op() == WorkflowOp::UpdateAssets {
        workflow::update_assets_metadata(dbm_mddb, meta_data).await;
    }

    if meta_data_ops.exchanges_op() == WorkflowOp::UpdateExchanges {
        workflow::update_exchanges_metadata(dbm_mddb, meta_data).await;
    }

    if meta_data_ops.instruments_op() == WorkflowOp::UpdateInstruments {
        workflow::update_instruments_metadata(dbm_mddb, meta_data).await;
    }
}
