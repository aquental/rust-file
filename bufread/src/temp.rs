use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("temperatures.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut total_sum = 0;
    let mut count = 0;

    // Read each line from the file
    for line in reader.lines() {
        // Parse the line to an integer
        let temperature = line.unwrap().parse::<i32>().unwrap();
        // Update total_sum and count
        total_sum += temperature;
        count += 1;
    }

    // Calculate and print the average temperature if count > 0
    if count > 0 {
        // Calculate the average
        let average = total_sum as f64 / count as f64;
        // Output the result
        println!("Average temperature: {}", average);
    } else {
        println!("No temperatures recorded.");
    }

    Ok(())
}
