mod metadata;
mod specs;

// Re-export insert query generators
// meta data
pub use crate::insert::metadata::meta_asset_insert::generate_asset_insert;
pub use crate::insert::metadata::meta_exchange_insert::generate_exchange_insert;
pub use crate::insert::metadata::meta_instruments_insert::generate_instruments_insert;
pub use crate::insert::metadata::meta_symbols_insert::generate_master_symbols_insert;
// specs
pub use crate::insert::specs::service_insert::generate_all_service_insert;
pub use crate::insert::specs::trades_insert::generate_trades_insert_query;
pub use crate::insert::specs::z_meta_insert::generate_meta_data_insert_query;
