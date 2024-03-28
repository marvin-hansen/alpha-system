use crate::gen_ddl;
use crate::gen_query;
use klickhouse::Client;
use lib_import::types::count::CountRow;
use std::error::Error;

/// Creates a table in the ClickHouse database with the specified table name.
///
/// # Arguments
///
/// * `client` - The ClickHouse client used to interact with the database.
/// * `table_name` - The name of the table to create.
///
/// # Returns
///
/// Returns `Ok(())` if the table is successfully created, or `Err` if an error occurs.
///
/// # Example
///
/// ```
/// use klickhouse::Client;
/// use std::error::Error;
///
/// # async fn example() -> Result<(), Box<dyn Error>> {
/// let client = Client::default();
/// let table_name = "default.services";
///
/// create_table(&client, table_name).await?;
/// # Ok(())
/// # }
/// ```
pub(crate) async fn create_table(client: &Client, table_name: &str) -> Result<(), Box<dyn Error>> {
    let query = gen_ddl::generate_services_table_ddl(table_name);

    client
        .execute(&query)
        .await
        .expect("[create_table]: Failed to create table");

    Ok(())
}

/// Counts the number of rows in the specified table in the ClickHouse database.
///
/// # Arguments
///
/// * `client` - The ClickHouse client used to interact with the database.
/// * `table_name` - The name of the table to count the rows from.
///
/// # Returns
///
/// Returns the number of rows in the specified table if the count is successful, or `Err` if an error occurs.
///
/// # Example
///
/// ```
/// use klickhouse::Client;
/// use std::error::Error;
///
/// # async fn example() -> Result<u64, Box<dyn Error>> {
/// let client = Client::default();
/// let table_name = "default.services";
///
/// let row_count = count_rows(&client, table_name).await?;
/// # Ok(())
/// # }
/// ```
pub(crate) async fn count_rows(client: &Client, table_name: &str) -> Result<u64, Box<dyn Error>> {
    let count_query = gen_query::generate_count_services(table_name);

    let number_of_rows: CountRow = client
        .query_one(&count_query)
        .await
        .expect("[count_rows]: Failed to count rows in table");

    Ok(number_of_rows.count())
}

/// Inserts data into the specified table in the ClickHouse database.
///
/// # Arguments
///
/// * `client` - The ClickHouse client used to interact with the database.
/// * `table_name` - The name of the table to insert the data into.
///
/// # Returns
///
/// Returns `Ok(())` if the data is successfully inserted, or `Err` if an error occurs.
///
/// # Example
///
/// ```
/// use klickhouse::Client;
/// use std::error::Error;
///
/// # async fn example() -> Result<(), Box<dyn Error>> {
/// let client = Client::default();
/// let table_name = "default.services";
///
/// insert_data(&client, table_name).await?;
/// # Ok(())
/// # }
/// ```
pub(crate) async fn insert_data(client: &Client, table_name: &str) -> Result<(), Box<dyn Error>> {
    let query = gen_query::generate_all_service_insert(table_name);

    client
        .execute(&query)
        .await
        .expect("[insert_data]: Failed to create trade table");

    Ok(())
}
