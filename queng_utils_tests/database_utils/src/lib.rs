use common_database::prelude::PostgresDBSchema;
use common_exchange::prelude::Instrument as CommonInstrument;
use container_specs::postgres_container_specs::postgres_db_container_config;
use diesel::{Connection, PgConnection};
use docker_utils::DockerUtil;
use std::fmt::Error;

pub const DB_TEST_URL: &str = "postgres://postgres:postgres@localhost/postgres";

pub async fn postgres_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Sets up a postgres database for testing, runs the specified schema migration,
/// and then tears down the container.
///
/// # Arguments
///
/// * `schema` - The schema to run migrations for.
/// * `database_url` - The database url to connect to.
///
pub async fn postgres_test_setup(
    schema: PostgresDBSchema,
    database_url: &str,
) -> Result<(), Error> {
    // Create new DockerUtil
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Initiate CI container
    let container_config = postgres_db_container_config();
    docker_util
        .setup_container(&container_config)
        .await
        .expect("Failed to setup ci api proxy container");

    let conn = &mut postgres_connection(database_url).await;

    match schema {
        PostgresDBSchema::CMDB => {
            let result = pg_cmdb::run_cmdb_db_migration(conn);
            //dbg!(&result);
            assert!(result.is_ok());
            Ok(())
        }
        PostgresDBSchema::SMDB => {
            let result = pg_smdb::run_smdb_db_migration(conn);
            //dbg!(&result);
            assert!(result.is_ok());
            Ok(())
        }
        PostgresDBSchema::MDDB => {
            let result = pg_mddb::run_mddb_db_migration(conn);
            //dbg!(&result);
            assert!(result.is_ok());
            Ok(())
        }

        _ => {
            panic!("{}", format!("Unsupported schema: {:?}", schema))
        }
    }
}

pub fn get_instrument() -> CommonInstrument {
    CommonInstrument::new(
        "test_code".to_string(),
        "test_class".to_string(),
        "test_exchange_code".to_string(),
        "test_exchange_pair_code".to_string(),
        "test_base_asset".to_string(),
        "test_quote_asset".to_string(),
        Some("test".to_string()),
    )
}
