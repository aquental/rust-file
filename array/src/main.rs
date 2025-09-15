use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the path to the input file
    let file_path = Path::new("data/students_marks.txt");

    // Read all lines from the file into a string
    let content = fs::read_to_string(file_path)?;

    // Split the first line from 'content' into a vector of integers
    // - Use whitespace as a delimiter to split
    // - Convert the split parts into integers and store in a variable
    let first_line = content.lines().next().ok_or("File is empty")?;
    let numbers: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| {
            s.parse()
                .map_err(|e| format!("Failed to parse number: {}", e))
        })
        .collect::<Result<Vec<i32>, _>>()?;

    // Print the vector's contents
    // - Use println! to display the vector
    println!("{:?}", numbers);

    Ok(())
}
