use crate::model::instrument::Instrument;
use crate::model::portfolio::Portfolio;
use crate::model::portfolio_instrument::PortfolioInstrument;
use crate::Connection as PGConnection;
use common_exchange::PortfolioConfig as CommonPortfolioConfig;
use diesel::result::DatabaseErrorKind;
use diesel::{
    result::Error, BelongingToDsl, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl,
    SelectableHelper,
};

impl Portfolio {
    ///
    /// Reads and retrieves portfolio configuration data for a given portfolio ID from the database.
    ///
    /// # Arguments
    /// - `db`: Mutable reference to the `PGConnection` for the database operations.
    /// - `param_portfolio_id`: The ID of the portfolio to retrieve configuration data for.
    ///
    /// # Returns
    /// Result containing the `CommonPortfolioConfig` if successful, or a `QueryResult` error.
    ///
    pub fn read(
        db: &mut PGConnection,
        param_portfolio_id: i32,
    ) -> QueryResult<CommonPortfolioConfig> {
        match Self::check_if_portfolio_id_exists(db, param_portfolio_id) {
            Ok(exists) => {
                if !exists {
                    return Err(Error::DatabaseError(
                        DatabaseErrorKind::NotNullViolation,
                        Box::new(String::from(
                            "[Portfolio:read]: Portfolio ID does not exist and thus cannot be read",
                        )),
                    ));
                }
            }
            Err(e) => return Err(e),
        };

        // Reading data from many-to-many or m:n relations
        // https://diesel.rs/guides/relations.html

        let portfolio = match crate::schema::cmdb::portfolio::table
            .filter(crate::schema::cmdb::portfolio::portfolio_id.eq(param_portfolio_id))
            .select(Self::as_select())
            .get_result(db)
        {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        let instruments = match PortfolioInstrument::belonging_to(&portfolio)
            .inner_join(crate::schema::cmdb::instrument::table)
            .select(Instrument::as_select())
            .load(db)
        {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

        // Convert from Vec<PortfolioConfig> to Vec<CommonPortfolioConfig>
        Ok(Self::to_common_portfolio(&portfolio, &instruments))
    }
}
