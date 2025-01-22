use common_env::EnvironmentType;
use imdb_client::IMDBClientSelector;
use smdb_client::SMDBClientSelector;

#[tokio::test]
async fn test_select_smdb_client() {
    let host = "127.0.0.1".to_string();
    let port = 7070;

    // Cluster / Prod test would require a live connection to a running SMDB service.
    // However, this is a unit test, so we just mock the client.

    // Test with EnvironmentType::CI
    let client = client_utils::select_smdb_client(&EnvironmentType::CI, host.clone(), port).await;
    match client {
        SMDBClientSelector::SMDBCMockClient(_) => {}
        _ => {
            panic!("Expected SMDBCMockClient");
        }
    }

    // Test with other EnvironmentType
    let client =
        client_utils::select_smdb_client(&EnvironmentType::LOCAL, host.clone(), port).await;
    match client {
        SMDBClientSelector::SMDBCMockClient(_) => {}
        _ => {
            panic!("Expected SMDBCMockClient");
        }
    }

    // Test with EnvironmentType::UNKNOWN
    let client =
        client_utils::select_smdb_client(&EnvironmentType::UNKNOWN, host.clone(), port).await;
    match client {
        SMDBClientSelector::SMDBCMockClient(_) => {}
        _ => {
            panic!("Expected SMDBCMockClient");
        }
    }
}

#[tokio::test]
async fn test_select_imdb_client() {
    let host = "127.0.0.1".to_string();
    let port = 8080;

    // Cluster / Prod test would require a live live connection to a running IMDB service.
    // However, this is a unit test, so we just mock the client.

    // Test with EnvironmentType::CI
    let env = EnvironmentType::CI;
    let client = client_utils::select_imdb_client(&env, host.clone(), port).await;

    match client {
        IMDBClientSelector::IMDBCMockClient(_) => {}
        _ => panic!("Expected IMDBCMockClient"),
    }

    // Test with EnvironmentType::LOCAL
    let env = EnvironmentType::LOCAL;
    let client = client_utils::select_imdb_client(&env, host.clone(), port).await;
    match client {
        IMDBClientSelector::IMDBCMockClient(_) => {}
        _ => panic!("Expected IMDBCMockClient"),
    }

    // Test with EnvironmentType::UNKNOWN
    let env = EnvironmentType::UNKNOWN;
    let client = client_utils::select_imdb_client(&env, host.clone(), port).await;
    match client {
        IMDBClientSelector::IMDBCMockClient(_) => {}
        _ => panic!("Expected IMDBCMockClient"),
    }
}
