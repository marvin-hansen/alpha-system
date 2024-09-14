use crate::model::instrument::{CreateInstrument, Instrument};
use crate::model::portfolio::{CreatePortfolio, Portfolio, UpdatePortfolio};
use crate::model::portfolio_instrument::{CreatePortfolioInstrument, PortfolioInstrument};
use crate::schema::cmdb::instrument::dsl::instrument;
use crate::schema::cmdb::portfolio::dsl::*;
use crate::schema::cmdb::portfolio::table as portfolio_table;
use crate::schema::cmdb::portfolio_instrument::dsl::portfolio_instrument;
use crate::Connection as PGConnection;
use common_exchange::prelude::PortfolioConfig as CommonPortfolioConfig;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

impl Portfolio {
    /// Creates a new portfolio in the database.
    ///
    /// This function inserts a new portfolio and its associated instruments into the database.
    /// It first checks if the portfolio ID exists. If it does not, it returns a `DatabaseError`.
    /// Then, it starts a database transaction to insert the portfolio and its instruments.
    /// If any instrument does not exist, it is created before being associated with the portfolio.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations.
    /// * `data` - A reference to the `CommonPortfolioConfig` containing the portfolio data.
    ///
    /// # Workflow
    ///
    /// 1. Check if the portfolio ID exists in the database.
    /// 2. If the portfolio ID does not exist, return a DatabaseError.
    /// 3. Start a database transaction.
    /// 4. Insert the new portfolio into the database.
    /// 5. For each instrument in the portfolio, check if it exists; if not, create it.
    /// 6. Associate each instrument with the portfolio by creating a new portfolio_instrument relation.
    /// 7. Return the created portfolio configuration.
    ///
    /// # Returns
    ///
    /// * `QueryResult<CommonPortfolioConfig>` - The result of the query,
    /// containing the created portfolio configuration.
    ///
    /// # Errors
    ///
    /// * Returns a `DatabaseError` if the portfolio ID does not exist.
    /// * Returns any error encountered during the database transaction.
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
                .get_result::<Portfolio>(db)?;

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
            let common_portfolio = Portfolio::to_common_portfolio_with_common_instruments(
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
    /// Inserts each portfolio from the provided data into the database using the `Portfolio::create` method.
    ///
    /// # Arguments
    /// * `db` - A mutable reference to the `PGConnection` for database operations.
    /// * `data` - A slice of `CommonPortfolioConfig` containing the portfolio data to be inserted.
    ///
    /// # Returns
    /// * `QueryResult<()>` - Result indicating the success of inserting all portfolios.
    ///
    pub fn create_portfolio_collection(
        db: &mut PGConnection,
        data: &[CommonPortfolioConfig],
    ) -> QueryResult<()> {
        for pfc in data {
            match Portfolio::create(db, pfc) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    ///
    /// Reads all portfolio configurations from the database and returns them as a vector of CommonPortfolioConfig.
    ///
    /// # Arguments
    /// - `db`: Mutable reference to the PGConnection for the database operations.
    ///
    /// # Returns
    ///
    /// Result containing a vector of CommonPortfolioConfig if successful, or a QueryResult error.
    ///
    pub fn read_all(db: &mut PGConnection) -> QueryResult<Vec<CommonPortfolioConfig>> {
        let mut v = Vec::new();

        let res = match portfolio.load::<Portfolio>(db) {
            Ok(res) => {
                if res.is_empty() {
                    return Ok(v);
                }

                // consider using a parallel iterator here to improve performance
                for i in &res {
                    let p = Self::read(db, i.portfolio_id)?;
                    v.push(p)
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
    /// * `db` - A mutable reference to the `PGConnection` for database operations.
    ///
    /// # Returns
    ///
    /// * `QueryResult<u64>` - The total count of portfolios as a result of the query.
    ///
    pub fn count(db: &mut PGConnection) -> QueryResult<u64> {
        portfolio.count().get_result::<i64>(db).map(|c| c as u64)
    }

    ///
    /// Checks if a portfolio ID exists in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations.
    /// * `param_portfolio_id` - The ID of the portfolio to check for existence.
    ///
    /// # Returns
    ///
    /// * `QueryResult<bool>` - `true` if the portfolio ID exists, `false` otherwise.
    ///
    pub fn check_if_portfolio_id_exists(
        db: &mut PGConnection,
        param_portfolio_id: i32,
    ) -> QueryResult<bool> {
        match portfolio.find(param_portfolio_id).first::<Portfolio>(db) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    ///
    /// Updates a portfolio in the database if it exists.
    ///
    /// Checks if the portfolio exists; if not, returns an error. Updates the portfolio and its instruments within a transaction.
    ///
    /// # Arguments
    /// * `db` - A mutable reference to the `PGConnection` for database operations.
    /// * `param_portfolio_id` - The ID of the portfolio to update.
    /// * `data` - A reference to the `CommonPortfolioConfig` containing the updated portfolio data.
    ///
    /// # Workflow
    ///
    /// 1. Check if the portfolio exists
    /// 2. If the portfolio does not exist, return an `DatabaseError`
    /// 3. Start a database transaction
    /// 4. Update the portfolio in the database
    /// 5. For each instrument in the portfolio, check if it exists; if not, create it.
    /// 6. Check if the portfolio-instrument relation exists; if not, create it.
    /// 7. If the instrument exists, update it.
    /// 8. Commit the transaction and return the result.
    ///
    /// # Returns
    /// * `QueryResult<()>` - Result indicating the success of the update operation.
    ///
    /// # Errors
    /// * Returns a `DatabaseError` if the portfolio does not exist
    /// or any encountered error during the update process.
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
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///
    /// Deletes a portfolio and its associated instruments from the database if the portfolio exists.
    ///
    /// # Arguments
    ///
    /// * `db` - A mutable reference to the `PGConnection` for database operations.
    /// * `param_portfolio_id` - The ID of the portfolio to be deleted.
    ///
    /// # Workflow
    ///
    /// 1. Check if the portfolio ID exists using check_if_portfolio_id_exists.
    /// 2. If the portfolio does not exist, return a DatabaseError.
    /// 3. Start a database transaction.
    /// 4. Read all instruments associated with the portfolio.
    /// 5. Delete the portfolio.
    /// 6. Delete all associated instruments.
    /// 7. Commit the transaction and return the number of rows affected.
    ///
    /// # Returns
    ///
    /// * `QueryResult<usize>` - The number of rows affected by the deletion operation.
    /// If none are affected, the result will be `Ok(0)`.
    ///
    /// # Errors
    ///
    /// * Returns a `DatabaseError` if the portfolio ID does not exist or any error occurs during deletion.
    ///
    pub fn delete(db: &mut PGConnection, param_portfolio_id: i32) -> QueryResult<usize> {
        // Start transaction
        println!(
            "[Delete]: Start transaction to delete portfolio: {}",
            param_portfolio_id
        );
        match db.transaction(|db| {
            // Read all portfolio_instrument for portfolio
            let portfolio_instruments =
                match PortfolioInstrument::read_instruments_for_portfolio(db, param_portfolio_id) {
                    Ok(res) => res,
                    Err(e) => return Err(e),
                };

            // Only delete if portfolio_instrument that exists
            if portfolio_instruments.len() > 0 {
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
