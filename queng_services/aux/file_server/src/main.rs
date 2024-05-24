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
    let _meta_data = im
        .init()
        .await
        .expect("Failed to initialize FileServer service.");

    Ok(())
}
