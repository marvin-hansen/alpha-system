use crate::model::instrument::Instrument;
use crate::model::portfolio::{CreatePortfolio, Portfolio, UpdatePortfolio};
use crate::model::portfolio_instrument::CreatePortfolioInstrument;
use crate::schema::cmdb::portfolio::dsl::*;
use crate::schema::cmdb::portfolio_instrument::dsl::portfolio_instrument;
use crate::types::database_information::DatabaseErrorMessage;
use crate::Connection as PGConnection;
use common_exchange::prelude::PortfolioConfig as CommonPortfolioConfig;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

impl Portfolio {
    //
    pub fn create(
        db: &mut PGConnection,
        pfc: &CommonPortfolioConfig,
    ) -> QueryResult<CommonPortfolioConfig> {
        // Check if portfolio exists
        // if NOT, return an error, otherwise continue
        match Self::check_if_portfolio_id_exists(db, pfc.portfolio_id() as i32) {
            Ok(exists) => {
                if exists {
                    return Err(Error::DatabaseError(
                        DatabaseErrorKind::NotNullViolation,
                        Box::new(DatabaseErrorMessage::new(
                            "Portfolio ID already exist and can therefore cannot be inserted again",
                            "portfolio",
                        )),
                    ));
                }
            }
            Err(e) => return Err(e),
        };

        // Start transaction
        match db.transaction(|db| {
            let port_id: i32 = pfc.portfolio_id() as i32;

            let item = CreatePortfolio::from_common_portfolio(pfc);
            let instruments = pfc.portfolio_instruments();

            // Check for each instrument if it exists.
            // If so, continue
            // If not, insert it
            for create_instrument in instruments {
                match Instrument::check_if_instrument_code_exists(
                    db,
                    create_instrument.code().to_string(),
                ) {
                    Ok(exists) => {
                        if exists {
                            continue;
                        } else {
                            match Instrument::create(db, create_instrument) {
                                Ok(_) => {}
                                Err(e) => return Err(e),
                            };
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            // Then insert portfolio,
            let inserted_portfolio = match diesel::insert_into(portfolio)
                .values(item)
                .returning(Portfolio::as_returning())
                .get_result(db)
            {
                Ok(res) => res,
                Err(e) => return Err(e),
            };

            // Next, insert portfolio_instrument for each instrument
            for i in pfc.portfolio_instruments() {
                match diesel::insert_into(portfolio_instrument)
                    .values(&CreatePortfolioInstrument {
                        portfolio_id: port_id,
                        instrument_id: i.code().to_string(),
                    })
                    .execute(db)
                {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                };
            }

            // Finally, return the portfolio converted to a common portfolio
            Ok(inserted_portfolio.to_common_portfolio_with_instruments(instruments.to_owned()))
        }) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub fn count(db: &mut PGConnection) -> QueryResult<u64> {
        portfolio.count().get_result::<i64>(db).map(|c| c as u64)
    }

    pub fn check_if_portfolio_id_exists(
        db: &mut PGConnection,
        param_portfolio_id: i32,
    ) -> QueryResult<bool> {
        match portfolio.find(param_portfolio_id).first::<Portfolio>(db) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn read(db: &mut PGConnection, param_portfolio_id: i32) -> QueryResult<Self> {
        portfolio
            .filter(portfolio_id.eq(param_portfolio_id))
            .limit(1)
            .get_result::<Self>(db)
    }

    pub fn read_all(db: &mut PGConnection) -> QueryResult<Vec<Self>> {
        portfolio.load(db)
    }

    pub fn update(
        db: &mut PGConnection,
        param_portfolio_id: i32,
        pfc: &CommonPortfolioConfig,
    ) -> QueryResult<CommonPortfolioConfig> {
        // Check if portfolio exists
        // if NOT, return an error, otherwise continue
        match Self::check_if_portfolio_id_exists(db, param_portfolio_id) {
            Ok(exists) => {
                if !exists {
                    return Err(Error::DatabaseError(
                        DatabaseErrorKind::NotNullViolation,
                        Box::new(DatabaseErrorMessage::new(
                            "Portfolio ID DOES NOT exist and can therefore cannot be updated",
                            "portfolio",
                        )),
                    ));
                }
            }
            Err(e) => return Err(e),
        };

        // Start transaction
        match db.transaction(|db| {
            let item = UpdatePortfolio::from_common_portfolio(pfc);

            // Update portfolio
            let updated_portfolio = match diesel::update(portfolio.find(param_portfolio_id))
                .set(item)
                .returning(Portfolio::as_returning())
                .get_result::<Portfolio>(db)
            {
                Ok(res) => res,
                Err(e) => return Err(e),
            };

            let common_instruments = pfc.portfolio_instruments();
            for i in common_instruments {
                match Instrument::update(db, i.code().to_string(), i) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                };
            }

            Ok(updated_portfolio
                .to_common_portfolio_with_instruments(common_instruments.to_owned()))
        }) {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub fn delete(db: &mut PGConnection, param_portfolio_id: i32) -> QueryResult<usize> {
        diesel::delete(portfolio.filter(portfolio_id.eq(param_portfolio_id))).execute(db)
    }
}
