use crate::errors::InitError;
use crate::init::InitManager;

impl InitManager {
    pub(super) async fn init_level_1(
        &self,
        _valid_exchanges: Vec<String>,
    ) -> Result<(), InitError> {
        //

        Ok(())
    }
}
