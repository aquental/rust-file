use csv::Reader;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file_path = Path::new("data/data.csv");
    let file = File::open(file_path)?;

    // Create a CSV reader
    let mut reader = Reader::from_reader(file);

    // Initialize a HashSet to store unique occupations
    let mut occupations: HashSet<String> = HashSet::new();

    // Skip the header and process each remaining line
    for result in reader.records().skip(1) {
        let record = result?;
        // Add the occupation to the HashSet
        // - Use the `insert` method to add the occupation to the HashSet
        occupations.insert(record[2].to_string()); // Assuming occupation is in the third column (index 2)
    }

    // Print each unique occupation
    for occupation in occupations {
        println!("{}", occupation);
    }

    Ok(())
}
