use std::fmt::{Display, Formatter};
use std::time::Duration;

/// Represents the strategy to wait for a container to reach a certain state.
///
/// This enum defines different strategies to be used when waiting for a container
/// to be ready or to achieve a certain condition. It can be used to specify
/// how a container's readiness should be checked before considering it fully started
/// and operational.
///
/// Variants:
///
/// - `NoWait`: Do not wait. This variant indicates that the container should be considered
///   ready immediately after it is started, without any delay or additional checks.
///
/// - `WaitForDuration(duration: u64)`: Wait for a fixed duration. This variant takes a `u64`
///   value representing the number of seconds to wait before considering the container ready.
///   This strategy is useful when a container is known to take a certain time to initialize.
///
/// - `WaitUntilConsoleOutputContains(expected_output: String, timeout: u64)`: Wait until the container's
///   console output contains a specific string or until a timeout occurs. This variant takes a `String`
///   representing the expected output to wait for and a `u64` representing the timeout in seconds.
///   This strategy is useful for containers that emit a specific log message or signal when they are ready.
///
/// Examples:
///
/// ```
/// use common_container::WaitStrategy;
///
/// // No wait strategy, indicating immediate readiness.
/// let no_wait = WaitStrategy::NoWait;
///
/// // Wait for a fixed duration of 10 seconds.
/// let wait_for_duration = WaitStrategy::WaitForDuration(10);
///
/// // Wait until the console output contains "Ready" or timeout after 15 seconds.
/// let wait_until_output_contains = WaitStrategy::WaitUntilConsoleOutputContains("Ready".to_string(), 15);
/// ```
///
/// Note that the usage of these strategies depends on the specific requirements of the
/// container and the context in which it is being started.
///
#[derive(Debug, Default, Clone, Eq, PartialOrd, Ord, PartialEq, Hash)]
pub enum WaitStrategy {
    #[default]
    NoWait,
    WaitForDuration(u64),
    WaitUntilConsoleOutputContains(String, u64),
    WaitForHttpHealthCheck(String, Duration),
}

impl Display for WaitStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
