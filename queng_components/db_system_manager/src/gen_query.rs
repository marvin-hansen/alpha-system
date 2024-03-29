pub(crate) fn get_all_services_query() -> String {
    "SELECT * FROM system.services".to_string()
}

pub(crate) fn get_all_portfolios_query() -> String {
    "SELECT * FROM system.portfolios".to_string()
}
