use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    // Read the first line from the file
    reader.read_line(&mut line)?;
    // Print the first line to the console
    println!("{}", line.trim());

    Ok(())
}
