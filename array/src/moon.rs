use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to the input file
    let file_path = Path::new("data/moon_temp.txt");

    // Read all lines from the file into a string
    let content = fs::read_to_string(file_path)?;

    // Parse each line into integers
    // - Split each line by spaces
    // - Convert each split string into an integer
    // - Store parsed numbers in a vector of integer vectors
    let temperatures: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().map_err(|e| format!("Failed to parse number: {}", e)))
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()?;

    // Iterate over each 'day' in the temperatures vector, calculate the average of the day's temperatures
    for (i, row) in temperatures.iter().enumerate() {
        println!("Row {}: {:?}", i + 1, row);
    }
    
    for (i, day) in temperatures.iter().enumerate() {
        let sum: i32 = day.iter().sum();
        let count = day.len() as f64;
        let average = if count > 0.0 { sum as f64 / count } else { 0.0 };
        // Print the average temperature of the day
        println!("Day {} average temperature: {:.2}", i + 1, average);
    }

    Ok(())
}
