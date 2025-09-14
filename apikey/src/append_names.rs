use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_file_path = Path::new("data/output.txt");
    let names = vec!["James", "Bob", "Charlie"];

    // Open the file in append mode using OpenOptions
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(output_file_path)?;
    let mut writer = BufWriter::new(file);

    // Use a loop to go through the names vector and append each name to the file with a newline
    for name in names {
        writeln!(writer, "{}", name)?;
    }

    writer.flush()?;

    Ok(()) // Explicitly return Ok(())
}
