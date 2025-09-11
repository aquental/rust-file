use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("./input.txt"); // Define the correct file path for input.txt

    // Read the entire file content into a String
    let content = fs::read_to_string(file_path)?;

    // Output the file content
    println!("Full file content:");
    println!("{}", content);

    let data_path = Path::new("./data/data.txt"); // Define the correct file path for data.txt
    let content = fs::read_to_string(data_path)?;

    // Output the file content
    println!("Full file content:");
    println!("{}", content);

    Ok(())
}
