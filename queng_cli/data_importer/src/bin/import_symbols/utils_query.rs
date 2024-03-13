use klickhouse::{Client, KlickhouseError};
use lib_import::types::count::CountRow;
use std::error::Error;

pub(crate) async fn execute_query(client: &Client, query: &str) -> Result<(), Box<dyn Error>> {
    return match client.execute(query).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Failed to execute query: {}", query);
            Err(Box::from(e))
        }
    };
}

pub(crate) async fn execute_count_query(
    client: &Client,
    count_query: &str,
) -> Result<u64, Box<dyn Error>> {
    // We need this separation b/c of required type annotation of the Result type.
    let number_of_rows: Result<CountRow, KlickhouseError> = client.query_one(count_query).await;
    return match number_of_rows {
        Ok(number_of_rows) => Ok(number_of_rows.count()),
        Err(e) => {
            println!("Failed to execute query: {}", count_query);
            Err(Box::from(e))
        }
    };
}
