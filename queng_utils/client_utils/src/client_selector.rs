use common_env::EnvironmentType;
use imdb_client::*;
use smdb_client::*;

/// Select an `SMDBClientSelector` based on the given environment.
///
/// * `CLUSTER` will return an `SMDBClient`.
/// * `CI` will return an `SMDBCMockClient`.
/// * All other environments will return an `SMDBCMockClient`.
///
pub async fn select_smdb_client(
    env: &EnvironmentType,
    host: String,
    port: u16,
) -> SMDBClientSelector {
    match env {
        EnvironmentType::CLUSTER => SMDBClient::new(host.clone(), port).await.into(),
        EnvironmentType::CI => SMDBCMockClient::new(host.clone(), port).await.into(),
        _ => SMDBCMockClient::new(host.clone(), port).await.into(),
    }
}

/// Select an `IMDBClientSelector` based on the given environment.
///
/// * `CLUSTER` will return an `IMDBClient`.
/// * `CI` will return an `IMDBCMockClient`.
/// * All other environments will return an `IMDBCMockClient`.
///
pub async fn select_imdb_client(
    env: &EnvironmentType,
    host: String,
    port: u16,
) -> IMDBClientSelector {
    match env {
        EnvironmentType::CLUSTER => IMDBClient::new(host.clone(), port)
            .await
            .expect("Failed to create IMDB Client")
            .into(),
        EnvironmentType::CI => IMDBCMockClient::new(host.clone(), port)
            .await
            .expect("Failed to create IMDB Client")
            .into(),
        _ => IMDBCMockClient::new(host.clone(), port)
            .await
            .expect("Failed to create IMDB Client")
            .into(),
    }
}
