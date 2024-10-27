use crate::workflow::worflow_op::{MetaDataDBWOp, WorkflowOp, WorkflowOpAll};
use common_metadata::prelude::{MetaDataDBRecords, MetaDataSet};

/// This function determines the workflow operations needed to synchronize
/// metadata between the Kaiko data source and the Postgres database.
///
/// It evaluates the number of assets, exchanges, and instruments from both
/// the Kaiko dataset and the local database, setting appropriate workflow
/// operations based on the differences found.
///
/// Possible operations are:
/// * NoOP: No operations are needed.
/// * ImportAll: Import all assets, exchanges, and instruments from Kaiko.
/// * UpdatedAll: Update all assets, exchanges, and instruments in the database.
/// * PartialImport: Import only assets, exchanges, and instruments from Kaiko.
/// * PartialUpdate: Update only assets, exchanges, and instruments in the database.
///
/// Returns a `MetaDataDBWOp` struct containing the workflow operations
/// for all, assets, exchanges, and instruments.
///
pub(crate) async fn determine_workflow(
    meta_data_kaiko: &MetaDataSet,
    meta_data_db: &MetaDataDBRecords,
) -> MetaDataDBWOp {
    println!("workflow_dispatch");

    // Set all fields initially to no-op in case nothing else can be determined
    // For partial import or partial update, only the affected field will be marked
    // leaving all unaffected fields as no-op.
    let mut all_op: WorkflowOpAll = WorkflowOpAll::NoOPAll;
    let mut assets_op: WorkflowOp = WorkflowOp::NoOP;
    let mut exchanges_op: WorkflowOp = WorkflowOp::NoOP;
    let mut instruments_op: WorkflowOp = WorkflowOp::NoOP;

    let stats = meta_data_kaiko.stats();
    // Extract the number of assets, exchanges and instruments downloaded from Kaiko
    let kaiko_asset_count = stats.number_assets();
    let kaiko_exchange_count = stats.number_exchanges();
    let kaiko_instrument_count = stats.number_instruments();

    // Extract the number of assets, exchanges and instruments in the DB
    let db_asset_count = meta_data_db.number_db_assets();
    let db_exchange_count = meta_data_db.number_db_exchanges();
    let db_instrument_count = meta_data_db.number_db_instruments();

    // Nothing imported; return full import op
    if db_asset_count == 0 && db_exchange_count == 0 && db_instrument_count == 0 {
        all_op = WorkflowOpAll::ImportAll;
    }

    // Everything imported already, nothing to do here. Return no-op
    if db_asset_count == kaiko_asset_count
        && db_exchange_count == kaiko_exchange_count
        && db_instrument_count == kaiko_instrument_count
    {
        all_op = WorkflowOpAll::NoOPAll;
    }

    // Check for partial import
    if db_asset_count == 0 {
        all_op = WorkflowOpAll::ImportPartial;
        assets_op = WorkflowOp::ImportAssets;
    }

    if db_exchange_count == kaiko_exchange_count {
        all_op = WorkflowOpAll::ImportPartial;
        exchanges_op = WorkflowOp::ImportExchanges;
    }

    if db_instrument_count == kaiko_instrument_count {
        all_op = WorkflowOpAll::ImportPartial;
        instruments_op = WorkflowOp::ImportInstruments;
    }

    // Everything changed; return update all
    if (db_asset_count > 0)
        && (db_asset_count != kaiko_asset_count)
        && (db_exchange_count > 0)
        && (db_exchange_count != kaiko_exchange_count)
        && (db_instrument_count > 0)
        && (db_instrument_count != kaiko_instrument_count)
    {
        all_op = WorkflowOpAll::UpdateAll;
    }

    // Assets changed
    if (db_asset_count > 0) && (db_asset_count != kaiko_asset_count) {
        all_op = WorkflowOpAll::UpdatePartial;
        assets_op = WorkflowOp::UpdateAssets;
    }

    // Exchanges changed
    if (db_exchange_count > 0) && (db_exchange_count != kaiko_exchange_count) {
        all_op = WorkflowOpAll::UpdatePartial;
        exchanges_op = WorkflowOp::UpdateExchanges;
    }

    // Instruments changed
    if (db_instrument_count > 0) && (db_instrument_count != kaiko_instrument_count) {
        all_op = WorkflowOpAll::UpdatePartial;
        instruments_op = WorkflowOp::UpdateInstruments;
    }

    // Return the workflow ops for all, assets, exchanges and instruments
    MetaDataDBWOp::new(all_op, assets_op, exchanges_op, instruments_op)
}
