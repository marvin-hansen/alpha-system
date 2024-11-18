use common_data_bar::DataType;

/// Represents a command for data retrieval from Binance.
///
/// This enum provides different variants to start and stop data retrieval.
///
/// # Variants
///
/// - `Start(u32, Vec<String>, DataType)`: Starts data retrieval for a specific symbol.
///   - `u32`: The ID of the symbol.
///   - `Vec<String>`: A list of symbols to retrieve data for.
///   - [DataType](cci:4:///Users/marvin/RustroverProjects/quant-engine/queng_services_ims/ims_data/binance_data/src/binance_types/command.rs:0:0-7:0): The type of data to retrieve.
///
/// - `Stop(u32)`: Stops data retrieval for a specific symbol.
///   - `u32`: The ID of the symbol.
///
/// - `StopAll`: Stops all data retrieval.
///
pub enum BinanceDataCommand {
    Start(u32, Vec<String>, DataType),
    Stop(u32),
    StopAll,
}
