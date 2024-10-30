use diesel::r2d2::CustomizeConnection;
use diesel::{Connection, PgConnection};

/// A connection customizer designed for use in tests.
/// Implements CustomizeConnection in a way
/// that ensures transactions in a pool customized by it are never committed.
///
/// During testing, set pool size to 1 to ensure that the same test transaction
/// is re-used throughout the test.
///
/// See:
/// * https://github.com/diesel-rs/diesel/discussions/4323#discussioncomment-11087060
/// * https://docs.diesel.rs/master/diesel/r2d2/struct.TestCustomizer.html
#[derive(Debug)]
pub struct TestConnectionCustomizer;

impl<E> CustomizeConnection<PgConnection, E> for TestConnectionCustomizer {
    /// Starts a test transaction whenever a connection is acquired.
    fn on_acquire(&self, conn: &mut PgConnection) -> Result<(), E> {
        conn.begin_test_transaction()
            .expect("Failed to start test transaction");

        Ok(())
    }
}
