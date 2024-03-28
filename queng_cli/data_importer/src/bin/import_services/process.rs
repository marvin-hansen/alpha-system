use crate::{gen_ddl, gen_query};
use client_utils::print_utils;
use db_utils::query_utils;
use klickhouse::Client;
use std::error::Error;

/// Processes the data and inserts it into the specified table.
///
/// # Arguments
///
/// * `client` - The ClickHouse client used to interact with the database.
/// * `table_name` - The name of the table to insert the data into.
/// * `vrb` - A boolean indicating whether verbose output is enabled.
///
/// # Returns
///
/// Returns `Ok(())` if the data is successfully inserted, or `Err` if an error occurs.
///
pub(crate) async fn process(
    client: &Client,
    table_name: &str,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    //
    print_utils::dbg_print(vrb, "Create the data table if it doesn't exist");
    let ddl = gen_ddl::generate_services_table_ddl(table_name);
    query_utils::execute_query(client, &ddl)
        .await
        .expect("Failed to create table");

    print_utils::dbg_print(vrb, "Insert data into the table");
    let query = gen_query::generate_all_service_insert(table_name);
    query_utils::execute_query(client, &query)
        .await
        .expect("Failed to insert data into table");

    if vrb {
        print_utils::dbg_print(vrb, "Count the number of rows in the table");
        let number_of_rows = query_utils::count_rows(client, table_name)
            .await
            .expect("Failed to count rows in table");

        println!("Number of imported data: {}", number_of_rows);
    }

    print_utils::dbg_print(vrb, "Done!");

    Ok(())
}
