use crate::error::DownloadError;
use crate::fields::CEX_URL;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;

/// Scrapes the valid exchange names from a specified webpage.
///
/// This function sends an HTTP GET request to the webpage specified by `CEX_URL`.
/// It then parses the returned HTML to find and extract a list of exchange names
/// from a JavaScript array within a script tag. Duplicate entries are removed,
/// and the unique exchange names are returned in a sorted `Vec<String>`.
///
/// # Errors
///
/// Returns `DownloadError` if the request fails, the response cannot be read,
/// or the JSON cannot be deserialized.
///
pub(crate) async fn scrap_valid_exchanges(vrb: bool) -> Result<Vec<String>, DownloadError> {
    // The URL of the webpage to fetch
    let url = CEX_URL;

    // Create a new client
    let client = Client::new();

    // Fetch the webpage
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request")
        .text()
        .await
        .expect("Failed to read response");

    // Parse the HTML content
    let document = Html::parse_document(&response);
    let script_selector = Selector::parse("script").expect("Failed to parse selector 'Script'");

    // HashSet to store exchange names and remove duplicates
    let mut exchanges_set = HashSet::new();

    // Extract the JavaScript array
    for script in document.select(&script_selector) {
        let script_text = script.inner_html();
        if script_text.contains("cextabledata") {
            //
            let start = script_text
                .find("cextabledata")
                .expect("Failed to find cextabledata");
            // Find beginning of the data array
            let start = script_text[start..]
                .find('[')
                .expect("Failed to find '[' indicating beginning of array")
                + start;
            // Find end of the data array
            let end = script_text[start..]
                .find(']')
                .expect("Failed to find ']' indicating end of array")
                + start
                + 1;
            // Extract the content of the JSON array
            let json_str = &script_text[start..end];

            // Deserialize the JSON array
            let json_array: Vec<serde_json::Value> =
                serde_json::from_str(json_str).expect("Failed to deserialize JSON array");

            for item in json_array {
                if let Some(exchange) = item.get("Exchange") {
                    if let Some(name) = exchange.as_str() {
                        let cleaned_name = cleanup(name);
                        if cleaned_name != "BL3P" {
                            exchanges_set.insert(cleaned_name);
                        }
                    }
                }
            }
            break;
        }
    }

    // Convert the HashSet to a vector and sort it
    let mut exchanges: Vec<String> = exchanges_set.into_iter().collect();
    exchanges.sort();

    if vrb {
        println!("Scraped {} exchanges", exchanges.len());
        print_exchanges(&exchanges);
    }

    Ok(exchanges)
}

/// Function to print all exchanges
fn print_exchanges(exchanges: &Vec<String>) {
    for exchange in exchanges {
        println!("{}", exchange);
    }
}

/// Cleans up the exchange name by truncating any characters following the "&lt;" symbol.
///
/// This function checks if the input `name` contains the "&lt;" symbol and truncates
/// the string to remove this symbol and any characters that follow it. If the symbol
/// is not found, the original `name` is returned without modification.
///
/// # Arguments
///
/// * `name` - A string slice representing the name of the exchange to clean.
///
/// # Returns
///
/// Returns the cleaned up exchange name as a `String`.
fn cleanup(name: &str) -> String {
    if let Some(pos) = name.find("&lt;") {
        name[..pos].to_string()
    } else {
        name.to_string()
    }
}
