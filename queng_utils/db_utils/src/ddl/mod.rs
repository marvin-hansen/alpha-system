mod metadata;
mod specs;

// re-export DDl generators
// meta data
pub use metadata::meta_asset_ddl::generate_create_asset_table_ddl;
pub use metadata::meta_asset_ddl::generate_drop_asset_table_ddl;
//
pub use metadata::meta_exchange_ddl::generate_create_exchanges_table_ddl;
pub use metadata::meta_exchange_ddl::generate_drop_exchanges_table_ddl;

pub use metadata::meta_instrument_ddl::generate_instruments_table_ddl;
pub use metadata::meta_symbols_ddl::generate_master_symbols_table_ddl;
// specs
pub use specs::services_ddl::generate_services_table_ddl;
pub use specs::trades_ddl::generate_trades_table_ddl;
// Remove when not needed anymore
pub use specs::z_meta_ddl::generate_metadata_table_ddl;
