pub mod client_login;
pub mod client_logout;

pub mod data_start;
pub mod data_stop;
pub mod data_stop_all;
pub mod error_client;
pub mod error_data;
/// Module containing data subscription messages.
///
/// This includes messages like:
///
/// - `StartDataMessage`
/// - `StopDataMessage`
/// - `StartAllDataMessage`
/// - `StopAllDataMessage`
/// - `OHLCVBarMessage`
/// - `TradeBarMessage`
///
/// Grouping data subscription messages together keeps them organized
/// separately from client and error messages.
///
/// The data messages are exposed in the prelude for convenient importing.
///
/// # Exports
///
/// - `ohlcv_bar` - `OHLCVBarMessage`
/// - `start_data` - `StartDataMessage`
/// - `stop_data` - `StopDataMessage`
/// - `start_all_data` - `StartAllDataMessage`
/// - `stop_all_data` - `StopAllDataMessage`
/// - `trade_bar` - `TradeBarMessage`
///
pub mod ohlcv_bar;
pub mod trade;
pub mod trade_bar;
