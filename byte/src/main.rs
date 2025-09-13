use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("data/example.txt");
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    // Read and display first 100 bytes as a snippet
    println!("First 100 bytes snippet:");
    let mut buffer = vec![0u8; 100];
    let bytes_read = reader.read(&mut buffer)?;
    if bytes_read > 0 {
        let snippet = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("{}{}", snippet, if bytes_read == 100 { "..." } else { "" });
    } else {
        println!("File is empty");
    }

    // Reset reader to start of file for the original byte-by-byte reading
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Original functionality: read and print the first byte
    println!("\nReading the file byte by byte (first byte):");
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
