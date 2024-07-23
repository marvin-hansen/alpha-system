use common_container::prelude::WaitStrategy;

#[test]
fn test_wait_strategy_variants() {
    let never = WaitStrategy::Never;
    let wait_duration = WaitStrategy::WaitForDuration(60);

    assert_eq!(never, WaitStrategy::Never);
    assert_eq!(wait_duration, WaitStrategy::WaitForDuration(60));
}
