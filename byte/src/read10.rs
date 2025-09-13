use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn read_bytes_to_string(
    reader: &mut BufReader<File>,
    num_bytes: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut count = 0;

    while count < num_bytes {
        let mut byte = [0u8; 1]; // Read one byte at a time
        match reader.read(&mut byte) {
            Ok(0) => break, // End of file reached
            Ok(_) => {
                buffer.push(byte[0]);
                count += 1;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    let snippet = String::from_utf8_lossy(&buffer).to_string();
    Ok(snippet)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new("data/example.txt");
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    println!("Reading first 10 characters:");
    let snippet = read_bytes_to_string(&mut reader, 10)?;
    println!("{}", snippet);

    println!("\nReading next 10 characters:");
    let snippet = read_bytes_to_string(&mut reader, 10)?;
    println!("{}", snippet);

    println!(); // Add a newline after output
    Ok(())
}
