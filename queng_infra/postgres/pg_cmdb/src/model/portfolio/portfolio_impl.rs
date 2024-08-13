use crate::model::portfolio::{CreatePortfolio, Portfolio, UpdatePortfolio};
use crate::schema::cmdb::portfolio::dsl::*;
use crate::Connection;
use diesel::{
    insert_into, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper,
};
impl Portfolio {
    pub fn create(db: &mut Connection, item: &CreatePortfolio) -> QueryResult<Self> {
        // let in

        insert_into(portfolio)
            .values(item)
            .returning(Portfolio::as_returning())
            .get_result(db)
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
