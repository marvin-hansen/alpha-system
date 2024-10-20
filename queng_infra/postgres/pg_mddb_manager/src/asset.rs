use crate::PostgresMDDBManager;
use common_errors::prelude::PostgresDBError;
use common_metadata::prelude::MetaAsset;
use pg_mddb::prelude::Asset;

impl PostgresMDDBManager {
    pub async fn insert_asset(&self, asset: MetaAsset) -> Result<MetaAsset, PostgresDBError> {
        self.dbg_print("insert_assets");
        let conn = &mut self.get_connection();

        match Asset::create_asset(conn, asset) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    pub async fn insert_asset_collection(
        &self,
        assets: &[MetaAsset],
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("insert_asset_collection");
        let conn = &mut self.get_connection();

        match Asset::create_asset_collection(conn, assets) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::InsertFailed(e.to_string())),
        }
    }

    pub async fn count_assets(&self) -> Result<u64, PostgresDBError> {
        self.dbg_print("count_assets");
        let conn = &mut self.get_connection();

        match Asset::count(conn) {
            Ok(count) => Ok(count),
            Err(e) => Err(PostgresDBError::CountFailed(e.to_string())),
        }
    }

    pub async fn check_if_asset_id_exists(
        &self,
        asset_id: String,
    ) -> Result<bool, PostgresDBError> {
        self.dbg_print("check_if_asset_id_exists");
        let conn = &mut self.get_connection();

        match Asset::check_if_asset_id_exists(conn, asset_id) {
            Ok(exists) => Ok(exists),
            Err(e) => Err(PostgresDBError::CheckIfExistsFailed(e.to_string())),
        }
    }

    pub async fn read_asset(&self, asset_id: String) -> Result<MetaAsset, PostgresDBError> {
        self.dbg_print("read_asset");
        let conn = &mut self.get_connection();

        match Asset::read(conn, asset_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn read_all_assets(&self) -> Result<Vec<MetaAsset>, PostgresDBError> {
        self.dbg_print("read_all_assets");
        let conn = &mut self.get_connection();

        match Asset::read_all(conn) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::QueryFailed(e.to_string())),
        }
    }

    pub async fn update_asset(
        &self,
        asset_id: String,
        asset: MetaAsset,
    ) -> Result<usize, PostgresDBError> {
        self.dbg_print("update_asset");
        let conn = &mut self.get_connection();

        match Asset::update(conn, asset_id, asset) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::UpdateFailed(e.to_string())),
        }
    }

    pub async fn delete_asset(&self, asset_id: String) -> Result<usize, PostgresDBError> {
        self.dbg_print("delete_asset");
        let conn = &mut self.get_connection();

        match Asset::delete(conn, asset_id) {
            Ok(res) => Ok(res),
            Err(e) => Err(PostgresDBError::DeleteFailed(e.to_string())),
        }
    }
}
