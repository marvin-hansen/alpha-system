use crate::model::portfolio::{CreatePortfolio, Portfolio, UpdatePortfolio};
use crate::schema::cmdb::portfolio::dsl::*;
use crate::types::database_information::DatabaseErrorMessage;
use crate::Connection as PGConnection;
use common_exchange::prelude::PortfolioConfig as CommonPortfolioConfig;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};

impl Portfolio {
    pub fn create(db: &mut PGConnection, pfc: &CommonPortfolioConfig) -> QueryResult<Self> {
        // Check if portfolio exists; if so, return an error, otherwise continue
        match Self::check_if_portfolio_id_exists(db, pfc.portfolio_id() as i32) {
            Ok(exists) => {
                if exists {
                    return Err(Error::DatabaseError(
                        DatabaseErrorKind::NotNullViolation,
                        Box::new(DatabaseErrorMessage::new(
                            "Portfolio ID already exists and thus cannot be inserted again",
                            "portfolio",
                        )),
                    ));
                }
            }
            Err(e) => return Err(e),
        };

        let item = CreatePortfolio::from_common_portfolio(pfc);
        match diesel::insert_into(portfolio)
            .values(item)
            .returning(Portfolio::as_returning())
            .get_result(db)
        {
            Ok(res) => Ok(res),
            Err(e) => Err(e),
        }
    }

    pub fn read_all(db: &mut PGConnection) -> QueryResult<Vec<CommonPortfolioConfig>> {
        let mut v = Vec::new();

        let res = match portfolio.load::<Portfolio>(db) {
            Ok(res) => {
                if res.is_empty() {
                    return Ok(v);
                }

                for i in &res {
                    let p = Self::read(db, i.portfolio_id as i32)?;
                    v.push(p)
                }

                v
            }
            Err(e) => return Err(e),
        };

        Ok(res)
    }
    //

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

    pub fn update(
        db: &mut PGConnection,
        param_portfolio_id: i32,
        pfc: &CommonPortfolioConfig,
    ) -> QueryResult<()> {
        // Check if portfolio exists
        // if NOT, return an error, otherwise continue
        match Self::check_if_portfolio_id_exists(db, pfc.portfolio_id() as i32) {
            Ok(exists) => {
                if !exists {
                    return Err(Error::DatabaseError(
                        DatabaseErrorKind::NotNullViolation,
                        Box::new(DatabaseErrorMessage::new(
                            "Portfolio ID does not exist and thus cannot be updated",
                            "portfolio",
                        )),
                    ));
                }
            }
            Err(e) => return Err(e),
        };

        // If it exists, update it.
        let item = UpdatePortfolio::from_common_portfolio(pfc);
        match diesel::update(portfolio.find(param_portfolio_id))
            .set(item)
            .execute(db)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn delete(db: &mut PGConnection, param_portfolio_id: i32) -> QueryResult<usize> {
        diesel::delete(portfolio.filter(portfolio_id.eq(param_portfolio_id))).execute(db)
    }
}
