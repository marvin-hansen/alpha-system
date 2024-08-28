use crate::postgres_connection;
use common_database::prelude::PostgresDBSchema;
use container_specs::postgres_container_specs::postgres_db_container_config;
use docker_utils::DockerUtil;
use std::fmt::Error;

pub async fn postgres_full_setup(database_url: &str) -> Result<(), Error> {
    postgres_schema_setup(PostgresDBSchema::ALL, database_url).await
}

/// Sets up a postgres database for testing, runs the specified schema migration,
/// and then tears down the container.
///
/// # Arguments
///
/// * `schema` - The schema to run migrations for.
/// * `database_url` - The database url to connect to.
///
pub async fn postgres_schema_setup(
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
        PostgresDBSchema::ALL => {
            let result = pg_cmdb::run_cmdb_db_migration(conn);
            assert!(result.is_ok());
            let result = pg_smdb::run_smdb_db_migration(conn);
            assert!(result.is_ok());
            let result = pg_mddb::run_mddb_db_migration(conn);
            assert!(result.is_ok());
            Ok(())
        }
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
