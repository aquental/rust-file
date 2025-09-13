use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the path to the text file
    let file_path = Path::new("data/example.txt");

    // Function to read and print a specified number of bytes
    fn read_bytes(file_path: &Path, num_bytes: usize) -> Result<(), Box<dyn std::error::Error>> {
        // Open the file and create a BufReader
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        // Read the specified number of bytes and print them as characters
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
        println!("{}", snippet);
        Ok(())
    }

    println!("First 5 characters:");
    // TODO: Call read_bytes with 5
    read_bytes(file_path, 5)?;

    println!("\nFirst 10 characters:");
    // TODO: Call read_bytes with 10
    read_bytes(file_path, 10)?;

    println!("\nFirst 15 characters:");
    // TODO: Call read_bytes with 15
    read_bytes(file_path, 15)?;

    Ok(())
}
