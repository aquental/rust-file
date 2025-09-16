use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Specify the input file path
    let file_path = Path::new("data/data.csv");
    let file = File::open(file_path)?;

    // Create a CSV reader
    let mut reader = Reader::from_reader(file);

    // Initialize a vector to store names
    let mut names = Vec::new();

    // Iterate over each record, skipping the header
    for result in reader.records() {
        let record = result?;

        // Extract the name from the first column and push it to the names vector
        let name_field = record.get(0).unwrap_or_default().to_string();
        names.push(name_field);
    }

    // TODO: Sort the names vector alphabetically
    names.sort();

    // TODO: Output each name from the sorted vector
    // After the loop, print or otherwise use the data vector:
    println!("Parsed CSV Data:");
    for name in &names {
        println!("{}", name);
    }

    Ok(())
}
