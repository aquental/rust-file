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

    // TODO: Access the library name in the JSON data
    // - Store it in a variable
    let library_name = data["library"].as_str().unwrap_or("Unknown");

    // TODO: Output the library name to the console
    println!("Library Name: {}", library_name);

    Ok(())
}
