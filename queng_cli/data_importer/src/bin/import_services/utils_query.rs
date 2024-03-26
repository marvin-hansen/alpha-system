use crate::gen_query;
use klickhouse::Client;
use lib_import::types::count::CountRow;
use std::error::Error;

pub(crate) async fn count_rows(client: &Client) -> Result<u64, Box<dyn Error>> {
    let count_query = gen_query::generate_count_services();

    let number_of_rows: CountRow = client
        .query_one(&count_query)
        .await
        .expect("Failed to count rows in table");

    Ok(number_of_rows.count())
}
