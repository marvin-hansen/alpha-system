use crate::errors::FileServerError;
use crate::init::InitManager;

mod errors;
mod fields;
mod init;
mod service;
mod types;
mod utils;

const VRB: bool = true;

#[tokio::main]
async fn main() -> Result<(), Box<FileServerError>> {
    let im = InitManager::new(VRB);

    im.init().await.expect("Failed to initialize service.");

    Ok(())
}
