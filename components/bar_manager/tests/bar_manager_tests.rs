use bar_manager::BarManager;

#[test]
fn test_add_bars() {
    // Arrange
    let mut bar_manager = BarManager::new();
    let symbol = "AAPL";
    let bars = vec![/* test bars */];

    assert!(!bar_manager.has_data(symbol));

    // Act
    bar_manager.add_bars(symbol, bars.clone());

    // Assert
    assert!(bar_manager.has_data(symbol));
}

#[test]
fn test_remove_bars() {
    // Arrange
    let mut bar_manager = BarManager::new();
    let symbol = "AAPL";
    let bars = vec![/* test bars */];
    bar_manager.add_bars(symbol, bars.clone());
    assert!(bar_manager.has_data(symbol));

    // Act
    bar_manager.remove_bars(symbol);

    // Assert
    assert!(!bar_manager.has_data(symbol));
    assert!(bar_manager.get_bars(symbol).is_err());
}

#[test]
fn test_has_data() {
    // Arrange
    let mut bar_manager = BarManager::new();
    let symbol = "AAPL";

    // Act & Assert
    assert!(!bar_manager.has_data(symbol));

    bar_manager.add_bars(symbol, vec![/* test bars */]);
    assert!(bar_manager.has_data(symbol));
}

#[test]
fn test_get_bars() {
    // Arrange
    let mut bar_manager = BarManager::new();
    let symbol = "AAPL";
    let bars = vec![/* test bars */];
    bar_manager.add_bars(symbol, bars.clone());

    // Act
    let result = bar_manager.get_bars(symbol);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), bars);
}
