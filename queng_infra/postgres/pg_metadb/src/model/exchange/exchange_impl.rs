use crate::model::exchange::{CreateExchange, Exchange, UpdateExchange};
use crate::schema::mddb::exchanges::dsl::exchanges as exchanges_table;
use crate::schema::mddb::exchanges::exchange_id;
use crate::Connection;
use common_database::prelude::DatabaseErrorMessage;
use common_metadata::prelude::MetaExchange;
use diesel::result::Error;
use diesel::result::Error::DatabaseError;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

impl Exchange {
    /// Creates a new exchange in the database based on the provided MetaExchange information.
    ///
    /// # Arguments
    /// - `conn`: A mutable reference to the database connection.
    /// - `meta_exchange`: The MetaExchange struct containing the exchange information to be inserted.
    ///
    /// # Returns
    /// A Result containing the inserted MetaExchange if successful, or a diesel Error if an error occurs.
    ///
    pub fn create(
        conn: &mut Connection,
        meta_exchange: MetaExchange,
    ) -> Result<MetaExchange, Error> {
        let new_exchange = CreateExchange::from_meta_exchange(meta_exchange);
        match diesel::insert_into(exchanges_table)
            .values(&new_exchange)
            .get_result::<Exchange>(conn)
        {
            Ok(res) => Ok(res.to_meta_exchange()),
            Err(e) => Err(e),
        }
    }

    /// Creates a collection of exchanges in the database based on the provided metadata exchanges.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `meta_exchanges` - A vector of MetaExchange structs containing the metadata
    /// for the exchanges to be created.
    ///
    /// # Returns
    ///
    /// A Result indicating the success of the operation, where true represents successful creation
    /// and an Error represents a failure.
    pub fn create_exchange_collection(
        conn: &mut Connection,
        meta_exchanges: Vec<MetaExchange>,
    ) -> Result<bool, Error> {
        if meta_exchanges.is_empty() {
            return Err(DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(DatabaseErrorMessage::new(
                    "No exchanges provided. Collection is empty.",
                    "mddb.exchanges",
                )),
            ));
        }

        let new_exchanges: Vec<CreateExchange> = meta_exchanges
            .into_iter()
            .map(CreateExchange::from_meta_exchange)
            .collect();

        match diesel::insert_into(exchanges_table)
            .values(&new_exchanges)
            .execute(conn)
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// Retrieves the total count of exchanges from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Error>` - The total count of exchanges if successful, an error otherwise.
    ///
    pub fn count(conn: &mut Connection) -> Result<u64, Error> {
        exchanges_table
            .count()
            .get_result::<i64>(conn)
            .map(|c| c as u64)
    }

    /// Checks if an exchange with the given exchange ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `exchange_id_str` - The ID of the exchange to check for existence.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` indicating whether the exchange exists or not.
    ///
    pub fn check_if_exchange_id_exists(
        conn: &mut Connection,
        exchange_id_str: String,
    ) -> Result<bool, Error> {
        let exists = diesel::select(diesel::dsl::exists(
            exchanges_table.filter(exchange_id.eq(exchange_id_str)),
        ))
        .get_result(conn)?;
        Ok(exists)
    }

    /// Reads a MetaExchange from the database based on the provided exchange ID string.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `exchange_id_str` - A String representing the exchange ID to search for.
    ///
    /// # Returns
    ///
    /// A Result containing the retrieved MetaExchange if successful, or an Error if the operation fails.
    ///
    pub fn read(conn: &mut Connection, exchange_id_str: String) -> Result<MetaExchange, Error> {
        match exchanges_table
            .filter(exchange_id.eq(exchange_id_str))
            .first::<Exchange>(conn)
        {
            Ok(exchange) => Ok(exchange.to_meta_exchange()),
            Err(e) => Err(e),
        }
    }

    /// Reads all exchanges from the database and converts them to MetaExchange objects.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of MetaExchange objects if successful, or a diesel Error if an error occurs.
    ///
    pub fn read_all(conn: &mut Connection) -> Result<Vec<MetaExchange>, Error> {
        exchanges_table.load::<Exchange>(conn).map(|e| {
            e.into_iter()
                .map(|exchange| exchange.to_meta_exchange())
                .collect()
        })
    }

    /// Checks if an exchange with the given exchange ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `exchange_id_str` - The ID of the exchange to check for existence.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` indicating whether the exchange exists or not.
    ///
    pub fn update(
        conn: &mut Connection,
        exchange_id_str: String,
        meta_exchange: MetaExchange,
    ) -> Result<usize, Error> {
        let updated_exchange = UpdateExchange::from_meta_exchange(meta_exchange);
        diesel::update(exchanges_table.filter(exchange_id.eq(exchange_id_str)))
            .set(&updated_exchange)
            .execute(conn)
    }

    /// Deletes an exchange from the database by exchange ID.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `exchange_id_str` - The ID of the exchange to delete.
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult` containing the number of rows affected by the delete operation.
    /// If the exchange does not exist, the query will return `Ok(0)`.
    /// If the exchange exists and was deleted, the query will return `Ok(1)`.
    ///
    /// Note, delete only returns an error when either the database connection or the query fails.
    ///
    pub fn delete(conn: &mut Connection, exchange_id_str: String) -> Result<usize, Error> {
        diesel::delete(exchanges_table.filter(exchange_id.eq(exchange_id_str))).execute(conn)
    }
}
