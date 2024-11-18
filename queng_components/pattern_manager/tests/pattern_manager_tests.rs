use common_data_bar::OHLCVBar;
use common_trade::prelude::PatternType;
use pattern_manager::PatternManager;

#[test]
fn test_get_pattern_len() {
    let pattern_manager = PatternManager::new();

    let base_len = pattern_manager.get_pattern_len(&PatternType::Base).unwrap();
    assert_eq!(base_len, 43);

    let extra_len = pattern_manager
        .get_pattern_len(&PatternType::Extra)
        .unwrap();
    assert_eq!(extra_len, 139);

    let long_len = pattern_manager.get_pattern_len(&PatternType::Long).unwrap();
    assert_eq!(long_len, 9);

    let short_len = pattern_manager
        .get_pattern_len(&PatternType::Short)
        .unwrap();
    assert_eq!(short_len, 9);

    let err = pattern_manager.get_pattern_len(&PatternType::NullVal);
    assert!(err.is_err());
}

#[test]
fn test_get_eval_result() {
    let pattern_manager = PatternManager::new();

    let base_result = pattern_manager.get_eval_result(&PatternType::Base, 0);
    assert!(base_result.is_ok());

    let extra_result = pattern_manager.get_eval_result(&PatternType::Extra, 0);
    assert!(extra_result.is_ok());

    let long_result = pattern_manager.get_eval_result(&PatternType::Long, 0);
    assert!(long_result.is_ok());

    let short_result = pattern_manager.get_eval_result(&PatternType::Short, 0);
    assert!(short_result.is_ok());

    let null_result = pattern_manager.get_eval_result(&PatternType::NullVal, 0);
    assert!(null_result.is_err());
}

#[test]
fn test_update_patterns() {
    let pattern_manager = PatternManager::new();
    let window: [OHLCVBar; 6] = [
        OHLCVBar::default(),
        OHLCVBar::default(),
        OHLCVBar::default(),
        OHLCVBar::default(),
        OHLCVBar::default(),
        OHLCVBar::default(),
    ];

    let base_result = pattern_manager.update_patterns(&PatternType::Base, &window);
    assert!(base_result.is_ok());

    let extra_result = pattern_manager.update_patterns(&PatternType::Extra, &window);
    assert!(extra_result.is_ok());

    let long_result = pattern_manager.update_patterns(&PatternType::Long, &window);
    assert!(long_result.is_ok());

    let short_result = pattern_manager.update_patterns(&PatternType::Short, &window);
    assert!(short_result.is_ok());

    let null_result = pattern_manager.update_patterns(&PatternType::NullVal, &window);
    assert!(null_result.is_err());
}
