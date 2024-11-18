use common_data_bar::DataType;
use ims_common::BinanceDataCommand;

#[test]
fn test_data_command_variants() {
    // Test Start variant
    let start_command =
        BinanceDataCommand::Start(1, vec!["BTCUSDT".to_string()], DataType::TradeData);
    match start_command {
        BinanceDataCommand::Start(symbol_id, symbols, data_type) => {
            assert_eq!(symbol_id, 1);
            assert_eq!(symbols, vec!["BTCUSDT".to_string()]);
            assert_eq!(data_type, DataType::TradeData);
        }
        _ => panic!("Expected Start variant"),
    }

    // Test Stop variant
    let stop_command = BinanceDataCommand::Stop(2);
    match stop_command {
        BinanceDataCommand::Stop(symbol_id) => {
            assert_eq!(symbol_id, 2);
        }
        _ => panic!("Expected Stop variant"),
    }

    // Test StopAll variant
    let stop_all_command = BinanceDataCommand::StopAll;
    match stop_all_command {
        BinanceDataCommand::StopAll => {
            // No need to assert anything for StopAll variant
        }
        _ => panic!("Expected StopAll variant"),
    }
}
