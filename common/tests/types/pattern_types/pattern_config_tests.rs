use common::prelude::{PatternConfig, PatternType};

fn get_pattern_config() -> PatternConfig {
    PatternConfig::new(
        1,
        "test_name".to_string(),
        "test_description".to_string(),
        PatternType::Base,
        1,
        2,
        3,
        4,
    )
}

#[test]
fn tests_new() {
    let pattern_config = PatternConfig::new(
        1,
        "test_name".to_string(),
        "test_description".to_string(),
        PatternType::Base,
        1,
        2,
        3,
        4,
    );

    assert_eq!(pattern_config.pattern_config_id(), 1);
    assert_eq!(pattern_config.pattern_config_name(), "test_name");
    assert_eq!(
        pattern_config.pattern_config_description(),
        "test_description"
    );
    assert_eq!(pattern_config.pattern_type(), &PatternType::Base);
    assert_eq!(pattern_config.pattern_long_yes(), 1);
    assert_eq!(pattern_config.pattern_long_no(), 2);
    assert_eq!(pattern_config.pattern_short_yes(), 3);
    assert_eq!(pattern_config.pattern_short_no(), 4);
}

#[test]
fn tests_id() {
    let pattern_config = get_pattern_config();
    assert_eq!(pattern_config.pattern_config_id(), 1);
}

#[test]
fn test_name() {
    let pattern_config = get_pattern_config();
    assert_eq!(pattern_config.pattern_config_name(), "test_name");
}

#[test]
fn test_description() {
    let pattern_config = get_pattern_config();
    assert_eq!(
        pattern_config.pattern_config_description(),
        "test_description"
    );
}

#[test]
fn test_pattern_type() {
    let pattern_config = get_pattern_config();

    assert_eq!(pattern_config.pattern_type(), &PatternType::Base);
}

#[test]
fn test_pattern_long_yes() {
    let pattern_config = get_pattern_config();

    assert_eq!(pattern_config.pattern_long_yes(), 1);
}

#[test]
fn test_pattern_long_no() {
    let pattern_config = get_pattern_config();

    assert_eq!(pattern_config.pattern_long_no(), 2);
}

#[test]
fn test_pattern_short_yes() {
    let pattern_config = get_pattern_config();

    assert_eq!(pattern_config.pattern_short_yes(), 3);
}

#[test]
fn pattern_config_debug() {
    let pattern_config = get_pattern_config();
    assert_eq!(
        format!("{:?}", pattern_config),
        "PatternConfig { pattern_config_id: 1, pattern_config_name: \"test_name\", pattern_config_description: \"test_description\", pattern_type: Base, pattern_long_yes: 1, pattern_long_no: 2, pattern_short_yes: 3, pattern_short_no: 4 }"
    );
}

#[test]
fn pattern_config_display() {
    let pattern_config = get_pattern_config();
    assert_eq!(
        format!("{}", pattern_config),
        "PatternConfig { pattern_config_id: 1, pattern_config_name: \"test_name\",pattern_config_description: \"test_description\", pattern_type: Base, pattern_long_yes: 1, pattern_long_no: 2, pattern_short_yes: 3, pattern_short_no: 4 }"
    );
}
