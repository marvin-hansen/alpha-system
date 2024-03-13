use crate::{process_assets, process_exchanges, process_instruments};
use klickhouse::Client;
use std::error::Error;
use std::path::PathBuf;

pub(crate) async fn process(
    client: &Client,
    file_path: &PathBuf,
    vrb: bool,
) -> Result<(), Box<dyn Error>> {
    //
    let file = file_path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to str")
        .replace(".json", "");

    match file.as_str() {
        "assets" => {
            process_assets::process_assets(client, file_path, vrb)
                .await
                .expect("Failed to process assets");
        }

        "exchanges" => {
            process_exchanges::process_exchanges(client, file_path, vrb)
                .await
                .expect("Failed to process exchanges");
        }

        "instruments" => {
            process_instruments::process_instruments(client, file_path, vrb)
                .await
                .expect("Failed to process instruments");
        }

        &_ => {
            println!("Unknown file: {}", file);
        }
    }

    Ok(())
}
