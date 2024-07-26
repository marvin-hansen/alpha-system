use std::env;
use std::time::Duration;

use tokio::process::Command;
use tokio::time::sleep;

use ctx_manager::CtxManager;
use env_utils::EnvUtil;

async fn setup_ci_env() {
    env::set_var("ENV", "CI");

    let env_util = EnvUtil::with_debug().await.expect("Failed to get EnvUtil");

    env_util
        .setup_postgres()
        .await
        .expect("Failed to setup postgres");
}

async fn setup_services() {
    let ctx_manager = CtxManager::with_debug();

    let program = "dbgw";
    let mut cmd = Command::new(program);

    let (env, val) = ctx_manager.env_var();
    cmd.env(env, val);

    println!("Executing command: {:?}", cmd);

    match cmd.output().await {
        Ok(out) => {
            println!(
                "[start_container]: \n
                    success: {} \n
                    Output: {}",
                out.status.success(),
                String::from_utf8_lossy(out.stdout.as_slice()),
            );
        }
        Err(e) => {
            panic!("Error starting binary {}: {}", program, e)
        }
    };

    sleep(Duration::from_secs(1)).await
}

#[tokio::test]
async fn test_core_services() {
    setup_ci_env().await;
    setup_services().await

    // let dbgw_client = DBGatewayClient::new().await;

    // run tests
}
