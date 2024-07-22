pub(crate) fn generate_verify_db_ddl(db_name: &str) -> String {
    format!(
        "SELECT EXISTS (
        SELECT
            datname
        FROM
            pg_catalog.pg_database
        WHERE
            datname = '{db_name}'
        );"
    )
}

// https://database.guide/5-ways-to-check-if-a-table-exists-in-postgresql/

pub(crate) fn generate_verify_table_ddl(schema_name: &str, table_name: &str) -> String {
    format!(
        "SELECT EXISTS (
        SELECT
            schemaname,
            tablename
        FROM
            pg_catalog.pg_tables
        WHERE
            schemaname = '{schema_name}'
            AND
            tablename  = '{table_name}'
    );"
    )
}

pub(crate) fn generate_verify_schema_ddl(schema_name: &str) -> String {
    format!(
        "SELECT EXISTS (
        SELECT
            nspname
        FROM
            pg_catalog.pg_namespace
        WHERE
            nspname = '{schema_name}');"
    )
}
