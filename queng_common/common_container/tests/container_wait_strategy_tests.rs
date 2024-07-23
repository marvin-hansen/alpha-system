use common_container::prelude::WaitStrategy;

#[test]
fn test_wait_strategy_variants() {
    let never = WaitStrategy::NoWait;
    let wait_duration = WaitStrategy::WaitForDuration(60);
    let wait_console_output =
        WaitStrategy::WaitUntilConsoleOutputContains("hello_world".to_string(), 30);

    assert_eq!(never, WaitStrategy::NoWait);
    assert_eq!(wait_duration, WaitStrategy::WaitForDuration(60));
    assert_eq!(
        wait_console_output,
        WaitStrategy::WaitUntilConsoleOutputContains("hello_world".to_string(), 30)
    );
}
