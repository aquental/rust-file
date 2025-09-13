use rand::Rng;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_file_path = Path::new("output.txt");

    // Write a random number to a file
    let random_number: i32 = rand::thread_rng().gen_range(0..100); // Generate random number in range 0 to 99
    let file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", random_number)?;
    writer.flush()?;
    println!("Random number written to {}", output_file_path.display());

    // Open the file for reading
    let file = File::open(output_file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    // Read the number back from the file
    reader.read_to_string(&mut content)?;

    // Print the read number
    println!("{}", content.trim());

    Ok(())
}
