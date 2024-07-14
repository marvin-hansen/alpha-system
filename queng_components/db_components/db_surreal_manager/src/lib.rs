mod db_cfg;
mod db_svc;
mod error;

use crate::error::SurrealDBError;
use common_config::prelude::SurrealDBConfig;
use std::fmt::{Debug, Display, Formatter};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Clone, Debug)]
pub struct SurrealDBManager {
    db: Surreal<Client>,
    db_config: SurrealDBConfig,
}

impl SurrealDBManager {
    pub async fn new(db_config: &SurrealDBConfig) -> Result<Self, SurrealDBError> {
        // Extract DB config parameters
        let db_host = db_config.host();
        let db_port = db_config.port();
        let db_address = format!("{}:{}", db_host, db_port);
        //
        let db_ns = db_config.db_namespace();
        let db_name = db_config.db_name();
        //
        let db_user = db_config.username();
        let db_pass = db_config.password();

        // Connect to the server
        let db = match Surreal::new::<Ws>(db_address).await {
            Ok(client) => client,
            Err(e) => return Err(SurrealDBError::ConnectionFailed(e.to_string())),
        };

        // Signin as a namespace, database, or user
        match db
            .signin(Root {
                username: db_user,
                password: db_pass,
            })
            .await
        {
            Ok(_) => (),
            Err(e) => return Err(SurrealDBError::LoginFailed(e.to_string())),
        };

        // Select a specific namespace / database
        db.use_db(db_name).await.expect("Failed to set db name");
        db.use_ns(db_ns).await.expect("Failed to set namespace");

        Ok(Self {
            db,
            db_config: db_config.to_owned(),
        })
    }
}

impl SurrealDBManager {
    pub async fn is_healthy(&self) -> Result<(), SurrealDBError> {
        match self.db.health().await {
            Ok(_) => Ok(()),
            Err(e) => Err(SurrealDBError::ConnectionFailed(e.to_string())),
        }
    }
}

impl Display for SurrealDBManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SurrealDBManager Connection parameter: {:?}",
            self.db_config
        )
    }
}
