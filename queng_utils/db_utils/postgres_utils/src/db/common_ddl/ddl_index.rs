// https://www.postgresql.org/docs/16/sql-dropindex.html

pub(crate) fn generate_drop_index_ddl(index_name: &str) -> String {
    format!("DROP INDEX IF EXISTS {index_name}")
}
