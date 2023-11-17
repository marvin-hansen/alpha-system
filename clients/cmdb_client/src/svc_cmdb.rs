use tarpc::context;
use tarpc::context::Context;

use common::prelude::CMDBError;

use crate::CMDBClient;

impl CMDBClient {
    async fn check_ok(&self, _: Context) -> Result<bool, CMDBError> {
        let res = self
            .client
            .check_ok(context::current())
            .await
            .expect("RPC failed to check ok");

        match res {
            Ok(res) => Ok(res),
            Err(e) => Err(CMDBError(e.to_string())),
        }
    }
}
