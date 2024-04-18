use clickhouse_rs::{Block, Pool};
use common::prelude::{ClickHouseConfig, EnvironmentType, ServiceID};
use config_manager::CfgManager;
use ctx_manager::CtxManager;
use dns_manager::DnsManager;
use std::env;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use test_utils::prelude::TestEnv;

#[tokio::test]
async fn test_new() {
    // Do the initial setup
    // Set the environment variable.
    env::set_var("ENV", "CI");
    // Internal CI DNS server.
    env::set_var("DNS_SERVER", "9.9.9.9");

    // Initialize the test environment to ensure all containers are up and running.
    let _test_env = TestEnv::setup_ci().expect("Failed to setup test env");
    // Give the container some extra time to complete initialization.
    // Otherwise, you may get a connection refused error. Adjust the time if needed.
    sleep(Duration::from_millis(700));
    // Build & configure components for contextual autoconfiguration.
    // Context manager determines the environment type.
    let ctxm = CtxManager::new();
    assert_eq!(ctxm.env_type(), EnvironmentType::CI);

    // Build & configure components for DNS autoconfiguration relative to the environment type.
    let dnm = DnsManager::new(&ctxm);
    assert_eq!(dnm.internal_dns(), "9.9.9.9:53");
    assert_eq!(dnm.external_dns(), "1.1.1.1:53");

    // Configure manager for context aware auto configuration.
    let config_manager = CfgManager::new(ServiceID::Default, &ctxm, &dnm);
    assert_eq!(config_manager.get_svc_id(), ServiceID::Default);
    assert_eq!(config_manager.get_env_type(), EnvironmentType::CI);

    let _clickhouse_config = ClickHouseConfig::new(
        "127.0.0.1".to_string(),
        9000,
        "default".to_string(),
        "".to_string(),
        "default".to_string(),
    );

    // Try a manual connect
    let out = Command::new("curl")
        .arg("-vso")
        .arg("/dev/null")
        .arg("--connect-timeout")
        .arg("5")
        .arg("localhost:8123")
        .output();

    assert!(out.is_ok());
    let out = out.unwrap();

    println!("status: {}", out.status);
    println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
    println!("stderr: stderr{}", String::from_utf8_lossy(&out.stderr));

    let out = Command::new("docker")
        .arg("exec")
        .arg("clickhouse-9000")
        .arg("cat")
        .arg("/var/log/clickhouse-server/clickhouse-server.err.log")
        .output();

    assert!(out.is_ok());
    let out = out.unwrap();

    println!("status: {}", out.status);
    println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
    println!("stderr: stderr{}", String::from_utf8_lossy(&out.stderr));

    let ddl = r"
        CREATE TABLE IF NOT EXISTS payment (
            customer_id  UInt32,
            amount       UInt32,
            account_name Nullable(FixedString(3))
        ) Engine=Memory";

    let block = Block::new()
        .column("customer_id", vec![1_u32, 3, 5, 7, 9])
        .column("amount", vec![2_u32, 4, 6, 8, 10])
        .column(
            "account_name",
            vec![Some("foo"), None, None, None, Some("bar")],
        );

    let dsn = "tcp://default:@127.0.0.1:9000/default";
    println!("✅: database_url");

    let pool = Pool::new(dsn);
    println!("✅: pool");
    println!("Pool config: {:?}", &pool);

    println!("Trying to get DB handle...");
    let mut client = pool.get_handle().await.expect("Failed to connect to DB");
    println!("✅: client");

    client
        .execute(ddl)
        .await
        .expect("Failed to execute DDL query");
    println!("✅: DDL");

    client
        .insert("payment", block)
        .await
        .expect("Failed to insert data");
    println!("✅: insert");

    let block = client
        .query("SELECT * FROM payment")
        .fetch_all()
        .await
        .expect("Failed to fetch data");
    println!("✅: query");

    for row in block.rows() {
        let id: u32 = row.get("customer_id").unwrap();
        let amount: u32 = row.get("amount").unwrap();
        println!("Found payment {}: {}", id, amount);
    }

    // let sdbm = SystemDBManager::new(&clickhouse_config).await;
    // assert!(sdbm.is_ok())
    // Unwrap the result and perform tests
}
