use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to the input file
    let file_path = Path::new("data/students_marks.txt");

    // Read all lines from the file into a string
    let content = fs::read_to_string(file_path)?;

    // Parse each line into integers
    // - Split each line by spaces
    // - Convert each split string into an integer
    // - Store parsed numbers in a vector of integer vectors
    let matrix: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse()
                        .map_err(|e| format!("Failed to parse number: {}", e))
                })
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()?;

    // Print each row line by line
    for (i, row) in matrix.iter().enumerate() {
        println!("Row {}: {:?}", i + 1, row);
    }

    Ok(())
}
