use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
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
            name: "Bob".to_string(),
            age: "23".to_string(),
            occupation: "Artist".to_string(),
        },
    ];

    // Specify the output file path
    let txt_file_path = Path::new("data/output.txt");
    let txt_file = File::create(txt_file_path)?;
    let mut txt_writer = BufWriter::new(txt_file);

    // Prepare data for tab-separated TXT file
    writeln!(txt_writer, "Name\tAge\tOccupation")?; // Header
    for person in &data {
        writeln!(
            txt_writer,
            "{}\t{}\t{}",
            person.name, person.age, person.occupation
        )?;
    }

    txt_writer.flush()?;
    println!(
        "Data successfully written to {} (tab-separated).",
        txt_file_path.display()
    );

    // Verify the content of the written file
    let file_content = std::fs::read_to_string(txt_file_path)?;
    println!("File content:");
    println!("{}", file_content);

    Ok(())
}
