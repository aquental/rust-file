use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("data/example.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Use bytes() to read the first byte and print it
    println!("Reading the file byte by byte (first byte):");
    if let Some(byte_result) = reader.bytes().next() {
        match byte_result {
            Ok(byte) => println!("First byte: {} (char: {})", byte, byte as char),
            Err(e) => {
                eprintln!("Error reading byte: {}", e);
                return Err(e.into());
            }
        }
    } else {
        println!("File is empty");
    }
    

    Ok(())
}
