use crate::all_dispatch::DataIntegration;
use crate::binance_data_integration::BinanceDataIntegration;
use crate::mock_data_integration::MockDataIntegration;
use common_ims::ExchangeDataIntegrationID;

/// Returns a `DataIntegration` instance based on the provided `ExchangeDataIntegration` enum value.
///
/// This function takes an `ExchangeDataIntegration` enum value as input and uses a `match` statement
/// to create a new instance of either `MockDataIntegration` or `BinanceDataIntegration`. The created
/// instance is then converted to a `DataIntegration` using the `from` method.
///
/// # Arguments
///
/// * `exchange_data_integration`: The `ExchangeDataIntegration` enum value that determines which type of
///   `DataIntegration` instance to create.
///
/// # Returns
///
/// A `DataIntegration` instance corresponding to the provided `ExchangeDataIntegration` enum value.
///
pub fn get_data_integration(
    exchange_data_integration: ExchangeDataIntegrationID,
) -> DataIntegration {
    match exchange_data_integration {
        ExchangeDataIntegrationID::BinanceData => {
            DataIntegration::from(BinanceDataIntegration::new())
        }
        ExchangeDataIntegrationID::MockData => DataIntegration::from(MockDataIntegration::new()),
    }
}
