mod binary;
mod commands;
mod traits;

pub use binary::binary_client::{BinaryClient, Client};
pub use binary::binary_client_state::ClientState;
pub use binary::binary_connection::ConnectionClient;
pub use binary::binary_protocol::BinaryProtocol;
pub use binary::binary_switch_client::SwitchClient;
pub use binary::binary_system_client::SystemClient;

pub mod binary_utils;

pub use commands::command_codes::*;
pub use commands::switch_commands::client_forward::ForwardMessage;
pub use commands::switch_commands::client_register::RegisterClient;
pub use commands::switch_commands::client_unregister::UnRegisterClient;
pub use commands::system_commands::{Heartbeat, Ping, Pong};

pub use traits::commandable::Commandable;
pub use traits::serializable::BytesSerializable;
pub use traits::sizeable::Sizeable;
pub use traits::validatable::Validatable;
