use std::fs::File;
use std::io::Write;

// Function to save exchanges to a JSON file
pub(crate) async fn save_to_json(
    exchanges: &Vec<String>,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the data vector to JSON
    let json =
        serde_json::to_string_pretty(exchanges).expect("Error converting data vector to JSON");

    // Create a file
    let mut file = File::create(filename).expect("Error creating JSON file");

    // Write the JSON to a file
    file.write_all(json.as_bytes())
        .expect("Error writing JSON to file");

    Ok(())
}
