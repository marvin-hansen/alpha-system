mod process;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //
    process::run().await.expect("Failed to process instruments");

    Ok(())
}
