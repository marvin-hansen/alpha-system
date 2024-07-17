pub(crate) fn generate_count_table_query(table_name: &str) -> String {
    format!("SELECT COUNT(*) FROM public.{table_name};")
}
