use serde_json::Value;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the JSON file
    let file_path = Path::new("data/students.json");

    // TODO: Read the JSON file into a string using fs::read_to_string
    let content = fs::read_to_string(file_path)?;

    // TODO: Parse the string to a serde_json::Value using serde_json::from_str
    let data: Value = serde_json::from_str(&content)?;

    // TODO: Access the "students" array second element
    // - Extract "name", "age", and "grade" as variables
    // - Print these values using println
    let name = data["students"][1]["name"].as_str().unwrap_or("Unknown");
    let age = data["students"][1]["age"].as_u64().unwrap_or(0);
    let grade = data["students"][1]["grade"].as_str().unwrap_or("Unknown");

    // TODO: Output the library name to the console
    println!("Student Name: {}, Age: {}, Grade: {}", name, age, grade);

    Ok(())
}
