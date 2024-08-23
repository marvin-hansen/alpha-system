use crate::model::portfolio::{CreatePortfolio, Portfolio, UpdatePortfolio};
use crate::schema::cmdb::portfolio::dsl::*;
use crate::Connection;
use common_exchange::prelude::PortfolioConfig as CommonPortfolioConfig;
use diesel::{
    insert_into, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper,
};

impl Portfolio {
    pub fn create(db: &mut Connection, item: &CreatePortfolio) -> QueryResult<Self> {
        // Insert instrument first, then portfolio, and then portfolio_instrument

        insert_into(portfolio)
            .values(item)
            .returning(Portfolio::as_returning())
            .get_result(db)
    }
    /// Retrieves the number of portfolios in the database.
    ///
    /// # Arguments
    ///
    /// * `db` - a mutable reference to a postgres database connection
    ///
    /// # Returns
    ///
    /// A `QueryResult<u64>` containing the number of portfolios,
    /// or an error if the operation fails.
    ///
    pub fn count(db: &mut Connection) -> QueryResult<u64> {
        portfolio.count().get_result::<i64>(db).map(|c| c as u64)
    }

    pub fn insert_portfolio_collection(
        _db: &mut Connection,
        _ports: &[CommonPortfolioConfig],
    ) -> QueryResult<bool> {
        // implement batch insert here

        Ok(false)
    }

    pub fn check_if_portfolio_id_exists(
        db: &mut Connection,
        param_portfolio_id: i32,
    ) -> QueryResult<bool> {
        match portfolio.find(param_portfolio_id).first::<Portfolio>(db) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn read(db: &mut Connection, param_portfolio_id: i32) -> QueryResult<Self> {
        portfolio
            .filter(portfolio_id.eq(param_portfolio_id))
            .limit(1)
            .get_result::<Self>(db)
    }

    pub fn read_all(db: &mut Connection) -> QueryResult<Vec<Self>> {
        portfolio.load(db)
    }

    pub fn update(
        db: &mut Connection,
        param_portfolio_id: i32,
        item: &UpdatePortfolio,
    ) -> QueryResult<Self> {
        diesel::update(portfolio.find(param_portfolio_id))
            .set(item)
            .returning(Portfolio::as_returning())
            .get_result(db)
    }

    pub fn delete(db: &mut Connection, param_portfolio_id: i32) -> QueryResult<usize> {
        diesel::delete(portfolio.filter(portfolio_id.eq(param_portfolio_id))).execute(db)
    }
}
