
use qd_manager::QDManager;
#[test]
fn test_get_data_bars() {
    let qd_manager = QDManager::new();

    // Add some test bars
    let result = qd_manager.get_data_bars("AAPL");

    assert!(result.is_ok());
}

