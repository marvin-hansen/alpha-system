use crate::service::Server;
use client_manager::ClientManager;
use common::errors::MessageProcessingError;
use sbe_messages::prelude::{ClientLoginMessage, ClientLogoutMessage};
use std::sync::{Arc, Mutex};

impl Server {
    pub(crate) async fn client_login(
        &self,
        client_manager: &Arc<Mutex<ClientManager>>,
        client_login_msg: &ClientLoginMessage,
    )
        -> Result<(), MessageProcessingError>
    {
        // Remove debug print
        println!("[QDGW/handle_client::client_login]: {:?}", client_login_msg);

        let mut client_db = client_manager.lock().unwrap();
        let exists = client_db.check_client(client_login_msg.client_id());
        if exists {
            return Err(MessageProcessingError(
                "[QDGW/handle_client::client_login]: Client already exists".to_string(),
            ));
        }

        client_db
            .add_client(client_login_msg.client_id(), client_login_msg.client_name())
            .expect("[QDGW/handle_client::client_login]: Failed to add client");

        // Remove debug print
        let exists = client_db.check_client(client_login_msg.client_id());
        println!("[QDGW/handle_client::client_login]: {:?}", exists);

        Ok(())
    }

    pub(crate) async fn client_logout(
        &self,
        client_manager: &Arc<Mutex<ClientManager>>,
        client_logout_msg: &ClientLogoutMessage,
    )
        -> Result<(), MessageProcessingError>
    {
        // Remove debug print
        println!(
            "[QDGW/handle_client::client_logout]: {:?}",
            client_logout_msg
        );

        let mut client_db = client_manager.lock().unwrap();
        client_db.remove_client(client_logout_msg.client_id());

        // Remove debug print
        let exists = client_db.check_client(client_logout_msg.client_id());
        println!("[QDGW/handle_client::client_login]: {:?}", exists);

        Ok(())
    }
}
