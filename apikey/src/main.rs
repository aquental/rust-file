use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "12345-abcde-67890-fghij";
    let output_file_path = Path::new("output.txt");

    // Create a new file and write the api_key to it using BufWriter
    let file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(api_key.as_bytes())?;
    writer.flush()?;

    println!(
        "api_key written to {} using 'write' mode.",
        output_file_path.display()
    );

    Ok(())
}
