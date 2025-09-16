use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;

struct Person {
    name: String,
    age: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file_path = Path::new("data/data.csv");
    let file = File::open(file_path)?;

    // Create a CSV reader
    let mut reader = Reader::from_reader(file);
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?; // Handle any potential CSV parsing error

        // record is a csv::StringRecord, which holds all fields in a row
        // We expect 2 fields: Name and Age

        // Extract fields by index (0 for name, 1 for age)
        let name_field = record.get(0).unwrap_or_default();
        let age_field = record.get(1).unwrap_or_default();

        // Convert age_field from string to integer
        let parsed_age: i32 = age_field.parse().unwrap_or(0);
        // Calculate future age by adding 10 years to the current age
        let future_age = parsed_age + 10;

        // Create a Person instance
        let person = Person {
            name: name_field.to_string(),
            age: future_age,
        };

        // Push into our data vector
        data.push(person);
    }

    // Display the result with the employee's name and calculated future age
    let p = data.get(0).unwrap();
    println!(
        "In 10 years, {} will be {} years old.",
        p.name,
        p.age
    );

    Ok(())
}
