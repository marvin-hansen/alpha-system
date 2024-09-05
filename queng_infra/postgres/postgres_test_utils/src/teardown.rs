use crate::get_or_wait_for_postgres_connection;
use common_database::prelude::PostgresDBSchema;
use container_specs::postgres_container_specs::postgres_db_container_config;
use docker_utils::DockerUtil;
use std::fmt::Error;

pub async fn postgres_full_teardown(database_url: &str) -> Result<(), Error> {
    postgres_schema_teardown(PostgresDBSchema::PostgresDBSchemaALL, database_url).await
}

pub async fn postgres_schema_teardown(
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

    let connection = get_or_wait_for_postgres_connection(database_url, Some(120)).await;
    assert!(connection.is_ok());

    let conn = &mut connection.unwrap();
    match schema {
        PostgresDBSchema::PostgresDBSchemaALL => {
            let result = pg_cmdb::revert_cmdb_db_migration(conn);
            assert!(result.is_ok());
            let result = pg_smdb::revert_smdb_db_migration(conn);
            assert!(result.is_ok());
            let result = pg_metadb::revert_metadb_db_migration(conn);
            //dbg!(&result);
            assert!(result.is_ok());
            Ok(())
        }
        PostgresDBSchema::PostgresDBSchemaCMDB => {
            let result = pg_cmdb::revert_cmdb_db_migration(conn);
            //dbg!(&result);
            assert!(result.is_ok());
            Ok(())
        }
        PostgresDBSchema::PostgresDBSchemaSMDB => {
            let result = pg_smdb::revert_smdb_db_migration(conn);
            //dbg!(&result);
            assert!(result.is_ok());
            Ok(())
        }
        PostgresDBSchema::PostgresDBSchemaMDDB => {
            let result = pg_metadb::revert_metadb_db_migration(conn);
            //dbg!(&result);
            assert!(result.is_ok());
            Ok(())
        }

        _ => {
            panic!("{}", format!("Unsupported schema: {:?}", schema))
        }
    }
}
