use serde_json::Value;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the JSON file
    let file_path = Path::new("data/data.json");

    // Read the entire content of the JSON file into a string
    let content = fs::read_to_string(file_path)?;

    // Parse the JSON content into a Value
    let data: Value = serde_json::from_str(&content)?;

    // Output the parsed JSON data
    println!("JSON Data:");
    let pretty_json = serde_json::to_string_pretty(&data)?;
    println!("{}", pretty_json);

    Ok(())
}
