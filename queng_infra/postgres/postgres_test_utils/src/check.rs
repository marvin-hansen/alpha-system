use common_database::prelude::PostgresDBSchema;
use std::env;
use std::fmt::Error;
use std::time::Instant;

#[inline(always)]
pub fn wait_until_schema_ready(schema: PostgresDBSchema, timeout: u64) -> Result<(), Error> {
    let start_time = Instant::now();

    loop {
        if start_time.elapsed().as_secs() > timeout {
            eprintln!(
                "!!Timeout!! Waited {} seconds for schema to be ready.",
                timeout
            );
            return Err(Error::default());
        }

        if check_schema_ready(schema)? {
            println!("Schema is ready.");
            break;
        } else {
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    }

    Ok(())
}

#[inline(always)]
pub fn check_schema_ready(schema: PostgresDBSchema) -> Result<bool, Error> {
    match schema {
        PostgresDBSchema::ALL => Ok(check_env_var("ALL_MIGRATION_READY")),
        PostgresDBSchema::Postgres => Ok(check_env_var("POSTGRES_MIGRATION_READY")),
        PostgresDBSchema::SMDB => Ok(check_env_var("SMDB_MIGRATION_READY")),
        PostgresDBSchema::CMDB => Ok(check_env_var("CMDB_MIGRATION_READY")),
        PostgresDBSchema::IMDB => Ok(check_env_var("IMDB_MIGRATION_READY")),
        PostgresDBSchema::MDDB => Ok(check_env_var("MDDB_MIGRATION_READY")),
    }
}

#[inline(always)]
fn check_env_var(key: &str) -> bool {
    match env::var(key) {
        Ok(res) => {
            if res == "True" {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
