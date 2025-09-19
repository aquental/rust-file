use serde_json::Value;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the JSON file
    let file_path = Path::new("data/products.json");

    // Read the entire content of the JSON file into a string
    let content = fs::read_to_string(file_path)?;

    // Parse the JSON content into a Value
    let data: Value = serde_json::from_str(&content)?;

    // Extract and print the name of the second product
    let second_product_name = data["products"][1]["name"].as_str().unwrap_or("Unknown");
    println!("Name of the Second Product: {}", second_product_name);

    Ok(())
}
