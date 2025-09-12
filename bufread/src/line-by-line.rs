use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reading text line-by-line
    let file_path = Path::new("input.txt");
    // Open the file using File::open
    let file = File::open(file_path)?;
    // Create a BufReader for the file
    let reader = BufReader::new(file);
    println!("Reading file line-by-line:");
    // Use a loop to read the file line-by-line using BufRead::lines()
    for line in reader.lines() {
        // Output each line to the console
        println!("{}", line?);
    }

    Ok(())
}
