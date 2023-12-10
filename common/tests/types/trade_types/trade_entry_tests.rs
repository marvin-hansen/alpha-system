use common::prelude::TradeEntryType;

#[test]
fn test_default_value() {
    let entry_type: TradeEntryType = Default::default();
    assert_eq!(entry_type, TradeEntryType::CurrentBar);
}

#[test]
fn test_display() {
    let entry_type = TradeEntryType::CurrentBar;
    assert_eq!(format!("{:?}", entry_type), "CurrentBar");
}

#[test]
fn test_next_bar() {
    let entry_type = TradeEntryType::NextBar;
    assert_eq!(entry_type, TradeEntryType::NextBar);
}

#[test]
fn test_current_bar() {
    let entry_type = TradeEntryType::CurrentBar;
    assert_eq!(entry_type, TradeEntryType::CurrentBar);
}

#[test]
fn test_next_bar_display() {
    let entry_type = TradeEntryType::NextBar;
    assert_eq!(entry_type.to_string(), "NextBar");
}

#[test]
fn test_current_bar_display() {
    let entry_type = TradeEntryType::CurrentBar;
    assert_eq!(entry_type.to_string(), "CurrentBar");
}
