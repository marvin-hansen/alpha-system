use common::prelude::{PatternConfig, StrategyConfig, TradeEntryType, TradeStrategyType};

fn get_strategy_config() -> StrategyConfig {
    let strategy_id = "test_id".to_string();
    let strategy_name = "test_name".to_string();
    let strategy_description = "test_description".to_string();
    let strategy_type = TradeStrategyType::BuyHold;
    let intraday = false;
    let day_to_filter = 1;
    let trade_entry_type = TradeEntryType::NextBar;
    let pattern_config = PatternConfig::default();
    let profit_target = 100;
    let stop_target = 50;
    let max_nr_bars = 10;

    StrategyConfig::new(
        strategy_id,
        strategy_name,
        strategy_description,
        strategy_type,
        intraday,
        day_to_filter,
        trade_entry_type,
        Some(pattern_config),
        profit_target,
        stop_target,
        max_nr_bars,
    )
}

#[test]
fn test_new() {
    let strategy_id = "test_id".to_string();
    let strategy_name = "test_name".to_string();
    let strategy_description = "test_description".to_string();
    let strategy_type = TradeStrategyType::BuyHold;
    let intraday = false;
    let day_to_filter = 1;
    let trade_entry_type = TradeEntryType::NextBar;
    let pattern_config = PatternConfig::default();
    let profit_target = 100;
    let stop_target = 50;
    let max_nr_bars = 10;

    let strategy_config = StrategyConfig::new(
        strategy_id,
        strategy_name,
        strategy_description,
        strategy_type,
        intraday,
        day_to_filter,
        trade_entry_type,
        Some(pattern_config),
        profit_target,
        stop_target,
        max_nr_bars,
    );

    assert_eq!(strategy_config.strategy_id(), &"test_id".to_string());
    assert_eq!(strategy_config.strategy_name(), &"test_name".to_string());
    assert_eq!(
        strategy_config.strategy_description(),
        &"test_description".to_string()
    );
    assert_eq!(strategy_config.strategy_type(), &TradeStrategyType::BuyHold);
    assert!(!strategy_config.intraday());
    assert_eq!(strategy_config.day_to_filter(), 1);
    assert_eq!(strategy_config.trade_entry_type(), &TradeEntryType::NextBar);
    assert_eq!(
        strategy_config.pattern_config(),
        &Some(PatternConfig::default())
    );
    assert_eq!(strategy_config.profit_target(), 100);
    assert_eq!(strategy_config.stop_target(), 50);
    assert_eq!(strategy_config.max_nr_bars(), 10);
}

#[test]
fn test_strategy_id() {
    let strategy_id = "test_id".to_string();
    let strategy_config = get_strategy_config();

    assert_eq!(strategy_config.strategy_id(), strategy_id.as_str());
}

#[test]
fn test_strategy_name() {
    let strategy_name = "test_name".to_string();
    let strategy_config = get_strategy_config();

    assert_eq!(strategy_config.strategy_name(), strategy_name.as_str());
}

#[test]
fn test_strategy_description() {
    let strategy_description = "test_description".to_string();
    let strategy_config = get_strategy_config();

    assert_eq!(
        strategy_config.strategy_description(),
        strategy_description.as_str()
    );
}

#[test]
fn test_strategy_type() {
    let strategy_type = TradeStrategyType::BuyHold;
    let strategy_config = get_strategy_config();

    assert_eq!(strategy_config.strategy_type(), &strategy_type);
}

#[test]
fn test_intraday() {
    let strategy_config = get_strategy_config();

    assert!(!strategy_config.intraday());
}

#[test]
fn test_day_to_filter() {
    let day_to_filter = 1;
    let strategy_config = get_strategy_config();

    assert_eq!(strategy_config.day_to_filter(), day_to_filter);
}

#[test]
fn test_trade_entry_type() {
    let trade_entry_type = TradeEntryType::NextBar;
    let strategy_config = get_strategy_config();

    assert_eq!(strategy_config.trade_entry_type(), &trade_entry_type);
}

#[test]
fn test_pattern_config() {
    let pattern_config = Some(PatternConfig::default());
    let strategy_config = get_strategy_config();

    assert_eq!(strategy_config.pattern_config(), &pattern_config);
}

#[test]
fn test_profit_target() {
    let profit_target = 100;
    let strategy_config = get_strategy_config();

    assert_eq!(strategy_config.profit_target(), profit_target);
}

#[test]
fn test_debug() {
    let strategy_config = get_strategy_config();
    let expected = "StrategyConfig { strategy_id: \"test_id\", strategy_name: \"test_name\", strategy_description: \"test_description\", strategy_type: BuyHold, intraday: false, day_to_filter: 1, trade_entry_type: NextBar, pattern_config: Some(PatternConfig { pattern_config_id: 0, pattern_config_name: \"\", pattern_config_description: \"\", pattern_type: Base, pattern_long_yes: 0, pattern_long_no: 0, pattern_short_yes: 0, pattern_short_no: 0 }), profit_target: 100, stop_target: 50, max_nr_bars: 10 }";
    let actual = format!("{:?}", strategy_config);

    assert_eq!(actual, expected);
}

#[test]
fn test_display() {
    let strategy_config = get_strategy_config();

    let expected = "StrategyConfig { strategy_id: test_id, strategy_name: test_name, strategy_description: test_description, strategy_type: BuyHold, intraday: false, day_to_filter: 1, trade_entry_type: NextBar, pattern_config: Some(PatternConfig { pattern_config_id: 0, pattern_config_name: \"\", pattern_config_description: \"\", pattern_type: Base, pattern_long_yes: 0, pattern_long_no: 0, pattern_short_yes: 0, pattern_short_no: 0 }), profit_target: 100, stop_target: 50, max_nr_bars: 10 }";
    let actual = strategy_config.to_string();
    assert_eq!(actual, expected);
}
