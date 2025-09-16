use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;

struct Person {
    name: String,
    age: i32,
    occupation: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define the file path
    let file_path = "data/data.csv";

    // Open the CSV file
    let file = File::open(&Path::new(file_path))?;

    // Create a CSV reader
    let mut reader = Reader::from_reader(file);

    // Initialize a vector to store Person structs
    let mut people: Vec<Person> = Vec::new();

    // Loop through each record in the CSV file
    for record in reader.records() {
        // Parse each line into fields
        let record = record?;

        // Create a Person struct from the fields
        let person = Person {
            name: record[0].to_string(),
            age: record[1].parse::<i32>()?,
            occupation: record[2].to_string(),
        };

        // Add the constructed Person struct to the vector
        people.push(person);
    }

    // Iterate through the vector of persons to output details
    for person in &people {
        println!(
            "Name: {}, Age: {}, Occupation: {}",
            person.name, person.age, person.occupation
        );
    }

    Ok(())
}
