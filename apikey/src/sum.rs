use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize file paths
    let input_file_path = Path::new("data/data.txt");
    let output_file_path = Path::new("data/result.txt");

    // Open the input file and create a buffered reader
    let input_file = File::open(input_file_path)?;
    let reader = BufReader::new(input_file);

    // Read numbers from data.txt and calculate their sum
    let mut sum = 0.0;
    for line in reader.lines() {
        let number = line?.trim().parse::<f64>()?;
        sum += number;
    }

    // Print the total sum
    println!("Total sum: {}", sum);

    // Open the output file and create a buffered writer
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file_path)?;
    let mut writer = BufWriter::new(output_file);

    // Write the total sum to result.txt
    writeln!(writer, "{}", sum)?;
    writer.flush()?;

    Ok(())
}
