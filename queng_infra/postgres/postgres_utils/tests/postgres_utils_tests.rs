use common_exchange::prelude::{AccountType, PortfolioConfig};
use container_specs::postgres_container_specs::postgres_db_container_config;
use docker_utils::DockerUtil;
use postgres_utils::PostgresUtil;

async fn setup_test() {
    // Create new DockerUtil
    let docker_util = DockerUtil::with_debug().expect("Failed to get DockerUtil");

    // Initiate CI container
    let container_config = postgres_db_container_config();
    docker_util
        .setup_container(&container_config)
        .await
        .expect("Failed to setup ci api proxy container");
}

async fn get_client() -> PostgresUtil {
    let dsn = "host=127.0.0.1 user=postgres password=postgres dbname=postgres";
    let res = PostgresUtil::with_debug(dsn).await;
    assert!(res.is_ok());

    res.unwrap()
}

#[tokio::test]
async fn postgres_db_test() {
    setup_test().await;

    let pg_util = get_client().await;

    //postgres_db_setup_test(&pg_util).await;

    //postgres_db_specs_import_test(&pg_util).await;

    // postgres_db_portfolio_import_test(&pg_util).await;

    postgres_db_teardown_test(&pg_util).await;
}
async fn postgres_db_setup_test(util: &PostgresUtil) {
    let res = util.setup_all_db().await;
    assert!(res.is_ok());
}

async fn postgres_db_specs_import_test(util: &PostgresUtil) {
    let svc = smdb_specs::smdb_service_config();
    let res = util.specs.insert_service(&svc).await;
    assert!(res.is_ok());
}
async fn postgres_db_portfolio_import_test(util: &PostgresUtil) {
    let config = get_portfolio_config();
    let res = util.specs.insert_portfolio(&config).await;
    assert!(res.is_ok());
}

async fn postgres_db_teardown_test(util: &PostgresUtil) {
    let res = util.teardown_all_db(false).await;
    assert!(res.is_ok());

    let res = util.drop_all_db().await;
    assert!(res.is_ok());
}

fn get_portfolio_config() -> PortfolioConfig {
    let portfolio_id = 1;
    let portfolio_description = "cash portfolio".to_string();
    let portfolio_account_type = AccountType::Spot;
    let portfolio_account_id = "cash_account".to_string();
    let portfolio_currency = "USD".to_string();
    let portfolio_cash_balance = 1000.0;
    let portfolio_max_drawdown = 20.0;
    let portfolio_instruments = vec![get_test_instrument()];
    let instrument_max_allocation = 0.0;
    let instrument_max_drawdown = 10.0;

    PortfolioConfig::new_cash_portfolio(
        portfolio_id,
        portfolio_description,
        portfolio_account_type,
        portfolio_account_id,
        portfolio_currency,
        portfolio_cash_balance,
        portfolio_max_drawdown,
        portfolio_instruments,
        instrument_max_allocation,
        instrument_max_drawdown,
    )
}

fn get_test_instrument() -> common_exchange::prelude::Instrument {
    common_exchange::prelude::Instrument::new(
        "ens-krw".to_string(),
        "spot".to_string(),
        "cbse".to_string(),
        "KRW-ENS".to_string(),
        "ens".to_string(),
        "krw".to_string(),
        None,
    )
}
