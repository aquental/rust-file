use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("data/example.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Use bytes() to read the first byte and print it
    println!("Reading the file byte by byte:");
    for byte_result in reader.bytes() {
        match byte_result {
            Ok(byte) => print!("{}", byte as char),
            Err(e) => {
                eprintln!("\nError reading byte: {}", e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}
