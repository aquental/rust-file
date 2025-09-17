use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

struct Person {
    name: String,
    age: String,
    occupation: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create data directory if it doesn't exist
    std::fs::create_dir_all("data")?;

    // Sample data to be written to the file
    let data = vec![
        Person {
            name: "John".to_string(),
            age: "28".to_string(),
            occupation: "Engineer".to_string(),
        },
        Person {
            name: "Alice".to_string(),
            age: "34".to_string(),
            occupation: "Doctor".to_string(),
        },
        Person {
            name: "Bobby".to_string(),
            age: "23".to_string(),
            occupation: "Artist".to_string(),
        },
    ];

    // Update the file path to "data/output.csv"
    let csv_file_path = Path::new("data/output.csv");
    let csv_file = File::create(csv_file_path)?;
    let buf_writer = BufWriter::new(csv_file);
    let mut csv_writer = Writer::from_writer(buf_writer);

    // Use the csv crate to write data as comma-separated values
    csv_writer.write_record(&["Name", "Age", "Occupation"])?; // header

    for person in &data {
        csv_writer.write_record(&[&person.name, &person.age.to_string(), &person.occupation])?;
    }

    csv_writer.flush()?;
    println!(
        "Data successfully written to {} (csv).",
        csv_file_path.display()
    );

    Ok(())
}
