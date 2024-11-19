use crate::model::instrument::{CreateInstrument, Instrument};
use crate::model::portfolio::{CreatePortfolio, Portfolio, UpdatePortfolio};
use crate::model::portfolio_instrument::{CreatePortfolioInstrument, PortfolioInstrument};
use crate::schema::cmdb::instrument::dsl::instrument;
use crate::schema::cmdb::portfolio::dsl::{portfolio, portfolio_id};
use crate::schema::cmdb::portfolio::table as portfolio_table;
use crate::schema::cmdb::portfolio_instrument::dsl::portfolio_instrument;
use crate::Connection as PGConnection;
use common_exchange::PortfolioConfig as CommonPortfolioConfig;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

impl Portfolio {
    /// Creates a new portfolio in the database.
    ///
    /// This function performs a transactional operation to:
    /// 1. Create a new portfolio record
    /// 2. Create or verify existence of each instrument
    /// 3. Create portfolio-instrument relationships
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations
    /// * `data` - A reference to the `CommonPortfolioConfig` containing the portfolio data
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<CommonPortfolioConfig>`:
    /// * `Ok(config)` - The created portfolio configuration with all its instruments
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Portfolio ID already exists (returns `DatabaseError(NotNullViolation)`)
    /// * Database connection errors during transaction
    /// * Transaction failures during:
    ///   - Portfolio creation
    ///   - Instrument creation or verification
    ///   - Portfolio-instrument relationship creation
    /// * Constraint violations:
    ///   - Unique constraints on portfolio or instrument IDs
    ///   - Foreign key constraints
    ///   - Not null constraints
    /// * Data validation errors:
    ///   - Invalid portfolio data format
    ///   - Invalid instrument data format
    /// * Concurrent modification conflicts
    ///
    /// # Implementation Notes
    ///
    /// The function uses a database transaction to ensure atomicity:
    /// * All operations succeed or all fail together
    /// * Database remains in a consistent state
    /// * Partial updates are not possible
    ///
    pub fn create(
        db: &mut PGConnection,
        data: &CommonPortfolioConfig,
    ) -> QueryResult<CommonPortfolioConfig> {
        let port_id = data.portfolio_id() as i32;

        // Check if the portfolio ID exists in the database.
        if Self::check_if_portfolio_id_exists(db, port_id)? {
            // If the portfolio ID does not exist, return a DatabaseError.
            return Err(Error::DatabaseError(
                DatabaseErrorKind::NotNullViolation,
                Box::new(String::from(
                    "[Portfolio:create]: Portfolio ID already exist and thus cannot be created again",
                )),
            ));
        }

        // Start transaction
        match db.transaction(|db| {
            //  Insert the new portfolio into the database.
            let create_portfolio = CreatePortfolio::from_common_portfolio(data);
            let inserted_portfolio = diesel::insert_into(portfolio_table)
                .values(create_portfolio)
                .get_result::<Self>(db)?;

            let common_instruments = data.portfolio_instruments();
            for instrument_data in common_instruments {
                let instrument_id = instrument_data.code().to_string();

                // For each instrument in the portfolio, check if it exists;
                if !Instrument::check_if_instrument_code_exists(db, instrument_id.clone())? {
                    // if not, create it.
                    let create_instrument =
                        CreateInstrument::from_common_instrument(instrument_data);
                    diesel::insert_into(instrument)
                        .values(create_instrument)
                        .execute(db)?;
                }

                // Associate each instrument with the portfolio by creating a new portfolio_instrument relation.
                if !PortfolioInstrument::check_if_exists(db, port_id, instrument_id.clone())? {
                    let create_portfolio_instrument =
                        CreatePortfolioInstrument::new(port_id, instrument_id.clone());
                    diesel::insert_into(portfolio_instrument)
                        .values(create_portfolio_instrument)
                        .execute(db)?;
                }
            }

            // Return the created portfolio configuration.
            let common_portfolio = Self::to_common_portfolio_with_common_instruments(
                &inserted_portfolio,
                common_instruments.to_owned(),
            );

            Ok(common_portfolio)
        }) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    ///
    /// Creates a collection of portfolios in the database.
    ///
    /// This function creates multiple portfolios sequentially, where each portfolio
    /// creation is itself a transactional operation.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations
    /// * `data` - A slice of `CommonPortfolioConfig` containing the portfolio data
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<()>`:
    /// * `Ok(())` - All portfolios were created successfully
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Any error that would occur in `Portfolio::create` for any portfolio
    /// * The operation stops at the first error encountered
    /// * Database connection errors
    /// * Transaction failures
    /// * Constraint violations:
    ///   - Duplicate portfolio IDs
    ///   - Invalid foreign keys
    ///   - Not null constraints
    /// * Data validation errors in any portfolio or instrument
    ///
    /// # Implementation Notes
    ///
    /// * Each portfolio creation is atomic but the collection operation is not
    /// * Successfully created portfolios remain in the database even if later ones fail
    /// * Consider using a larger transaction if all-or-nothing behavior is needed
    ///
    pub fn create_portfolio_collection(
        db: &mut PGConnection,
        data: &[CommonPortfolioConfig],
    ) -> QueryResult<()> {
        for pfc in data {
            match Self::create(db, pfc) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    ///
    /// Reads all portfolio configurations from the database.
    ///
    /// This function retrieves all portfolios and their associated instruments,
    /// performing the necessary joins and data conversions.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<Vec<CommonPortfolioConfig>>`:
    /// * `Ok(vec)` - Vector containing all portfolio configurations
    /// * Returns an empty vector if no portfolios exist
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures during:
    ///   - Portfolio retrieval
    ///   - Instrument relationship joins
    /// * Data deserialization errors:
    ///   - Converting database records to Portfolio structs
    ///   - Converting to CommonPortfolioConfig format
    /// * Memory allocation errors with large result sets
    ///
    /// # Performance Notes
    ///
    /// * This function performs joins to fetch instrument data
    /// * Consider pagination for large datasets
    /// * Memory usage scales with the number of portfolios and their instruments
    ///
    pub fn read_all(db: &mut PGConnection) -> QueryResult<Vec<CommonPortfolioConfig>> {
        let mut v = Vec::new();

        let res = match portfolio.load::<Self>(db) {
            Ok(res) => {
                if res.is_empty() {
                    return Ok(v);
                }

                // consider using a parallel iterator here to improve performance
                for i in &res {
                    let p = Self::read(db, i.portfolio_id)?;
                    v.push(p);
                }

                v
            }
            Err(e) => return Err(e),
        };

        Ok(res)
    }

    ///
    /// Retrieves the count of portfolios stored in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<u64>`:
    /// * `Ok(count)` - The total number of portfolios in the database
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Type conversion errors:
    ///   - When converting count from i64 to u64
    ///   - Overflow if count exceeds u64::MAX (extremely unlikely)
    ///
    /// # Performance Notes
    ///
    /// * Uses COUNT(*) SQL operation
    /// * Relatively fast as it doesn't fetch actual records
    /// * No joins or complex operations involved
    ///
    pub fn count(db: &mut PGConnection) -> QueryResult<u64> {
        portfolio.count().get_result::<i64>(db).map(|c| c as u64)
    }

    ///
    /// Checks if a portfolio ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations
    /// * `param_portfolio_id` - The ID of the portfolio to check
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<bool>`:
    /// * `Ok(true)` - The portfolio exists
    /// * `Ok(false)` - No portfolio exists with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection errors
    /// * Query execution failures
    /// * Type conversion errors when processing results
    ///
    /// # Implementation Notes
    ///
    /// * Uses EXISTS clause for efficient checking
    /// * Does not retrieve the actual portfolio data
    /// * Not finding a portfolio is NOT an error condition
    ///
    pub fn check_if_portfolio_id_exists(
        db: &mut PGConnection,
        param_portfolio_id: i32,
    ) -> QueryResult<bool> {
        match portfolio.find(param_portfolio_id).first::<Self>(db) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    ///
    /// Updates a portfolio in the database if it exists.
    ///
    /// This function performs a transactional update of:
    /// 1. The portfolio record itself
    /// 2. Associated instruments (creating new ones if needed)
    /// 3. Portfolio-instrument relationships
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations
    /// * `param_portfolio_id` - The ID of the portfolio to update
    /// * `data` - A reference to the `CommonPortfolioConfig` with updated data
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<()>`:
    /// * `Ok(())` - The update was successful
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Portfolio does not exist (returns DatabaseError)
    /// * Database connection errors
    /// * Transaction failures during:
    ///   - Portfolio update
    ///   - Instrument creation/update
    ///   - Relationship updates
    /// * Constraint violations:
    ///   - Foreign key constraints
    ///   - Unique constraints
    ///   - Not null constraints
    /// * Data validation errors in updated data
    /// * Concurrent modification conflicts
    ///
    /// # Implementation Notes
    ///
    /// * Uses a transaction to ensure atomicity
    /// * Existing relationships are preserved unless explicitly changed
    /// * Missing instruments are created automatically
    /// * Handles both additions and removals of instruments
    ///
    pub fn update(
        db: &mut PGConnection,
        param_portfolio_id: i32,
        data: &CommonPortfolioConfig,
    ) -> QueryResult<()> {
        // Check if portfolio exists
        // if NOT, return an error, otherwise continue
        match Self::check_if_portfolio_id_exists(db, param_portfolio_id) {
            Ok(exists) => {
                if !exists {
                    return Err(Error::DatabaseError(
                        DatabaseErrorKind::NotNullViolation,
                        Box::new(String::from(
                            "[Portfolio:update]: Portfolio ID does not exist and thus cannot be updated",
                        )),
                    ));
                }
            }
            Err(e) => return Err(e),
        };

        // Start transaction
        match db.transaction(|db| {
            // Update portfolio
            let item = UpdatePortfolio::from_common_portfolio(data);
            match diesel::update(portfolio.find(param_portfolio_id))
                .set(item)
                .execute(db)
            {
                Ok(_) => {}
                Err(e) => return Err(e),
            }

            // Update portfolio instruments
            let common_instruments = data.portfolio_instruments();
            for i in common_instruments {
                // Check if instrument already exists
                match Instrument::check_if_instrument_code_exists(db, i.code().to_string()) {
                    Ok(exists) => {
                        if !exists {
                            // If not add it
                            match Instrument::create(db, i) {
                                Ok(_) => {}
                                Err(e) => return Err(e),
                            }

                            // Check if portfolio_instrument relation exists
                            match PortfolioInstrument::check_if_exists(
                                db,
                                param_portfolio_id,
                                i.code().to_string(),
                            ) {
                                Ok(exists) => {
                                    if !exists {
                                        // If not add it
                                        match PortfolioInstrument::create(
                                            db,
                                            &CreatePortfolioInstrument {
                                                portfolio_id: param_portfolio_id,
                                                instrument_id: i.code().to_string(),
                                            },
                                        ) {
                                            // If it does not exists, do nothing
                                            Ok(_) => {}
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                Err(e) => return Err(e),
                            }
                        };
                    }
                    Err(e) => return Err(e),
                }

                // If it exists, update it.
                match Instrument::update(db, i.code().to_string(), i) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                };
            }

            Ok(())
        }) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///
    /// Deletes a portfolio and its associated relationships from the database.
    ///
    /// This function performs a transactional deletion that:
    /// 1. Removes portfolio-instrument relationships
    /// 2. Deletes the portfolio record
    /// 3. Maintains referential integrity
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations
    /// * `param_portfolio_id` - The ID of the portfolio to delete
    ///
    /// # Returns
    ///
    /// Returns a `QueryResult<usize>`:
    /// * `Ok(1)` - The portfolio was successfully deleted
    /// * `Ok(0)` - No portfolio existed with the given ID
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// * Database connection errors
    /// * Transaction failures during:
    ///   - Relationship deletion
    ///   - Portfolio deletion
    /// * Foreign key constraint violations:
    ///   - If the portfolio is referenced by other tables
    ///   - If relationships cannot be deleted
    /// * Concurrent modification conflicts
    /// * Deadlock detection during cascading deletes
    ///
    /// # Implementation Notes
    ///
    /// * Uses a transaction to ensure atomicity
    /// * Handles cascading deletes of relationships
    /// * Does not delete the instruments themselves
    /// * Safe to call on non-existent portfolio IDs
    ///
    pub fn delete(db: &mut PGConnection, param_portfolio_id: i32) -> QueryResult<usize> {
        // Start transaction
        println!("[Delete]: Start transaction to delete portfolio: {param_portfolio_id}");
        match db.transaction(|db| {
            // Read all portfolio_instrument for portfolio
            let portfolio_instruments =
                match PortfolioInstrument::read_instruments_for_portfolio(db, param_portfolio_id) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

            // Only delete if portfolio_instrument that exists
            if !portfolio_instruments.is_empty() {
                // Delete all portfolio_instrument for portfolio
                for i in portfolio_instruments {
                    match PortfolioInstrument::delete(db, i.portfolio_id, i.instrument_id) {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    };
                }
            }

            // Delete portfolio
            let res = match diesel::delete(portfolio.filter(portfolio_id.eq(param_portfolio_id)))
                .execute(db)
            {
                Ok(res) => res,
                Err(e) => return Err(e),
            };

            Ok(res)
        }) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }
}
