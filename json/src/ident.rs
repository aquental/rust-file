use serde_json::{Value, from_str, to_string_pretty};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the file path
    let input_file_path = "data/oneliner.json";

    // TODO: Read the contents of the JSON file and parse it, utilize File, BufReader, and serde_json
    let file = File::open(input_file_path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let json_data: Value = from_str(&contents)?;

    // TODO: Render the JSON data with indentation
    let indented_json_data = to_string_pretty(&json_data)?;

    // TODO: Write the indented JSON data back to event_data.json
    // Remember to flush!
    let mut writer = BufWriter::new(File::create(input_file_path)?);
    writer.write_all(indented_json_data.as_bytes())?;
    writer.flush()?;

    // Read the indented JSON data from event_data.json and print to console
    let verified_json_data = fs::read_to_string(input_file_path)?;
    println!("{}", verified_json_data);

    Ok(())
}
