use common::errors::MessageProcessingError;
use sbe_messages::prelude::{ClientLoginMessage, ClientLogoutMessage};
use crate::service::Server;

impl Server{

    pub(crate) async fn client_login(
        &self,
        client_login_msg: &ClientLoginMessage,
    ) -> Result<(), MessageProcessingError> {
        println!(
            "[QDGW/handle_client::client_login]: {:?}",
            client_login_msg
        );

        Ok(())
    }

    pub(crate) async fn client_logout(
        &self,
        client_logout_msg: &ClientLogoutMessage,
    ) -> Result<(), MessageProcessingError> {
        println!(
            "[QDGW/handle_client::client_logout]: {:?}",
            client_logout_msg
        );

        Ok(())
    }

}