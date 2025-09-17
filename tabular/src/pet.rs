use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

struct Pet {
    pet_type: String,
    name: String,
    age: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create data directory if it doesn't exist
    std::fs::create_dir_all("data")?;

    // Create a list of Pet objects
    let pets = vec![
        Pet {
            pet_type: "Cat".to_string(),
            name: "Whiskers".to_string(),
            age: "2".to_string(),
        },
        Pet {
            pet_type: "Dog".to_string(),
            name: "Fido".to_string(),
            age: "5".to_string(),
        },
        Pet {
            pet_type: "Bird".to_string(),
            name: "Tweety".to_string(),
            age: "1".to_string(),
        },
        Pet {
            pet_type: "Cat".to_string(),
            name: "Luna".to_string(),
            age: "3".to_string(),
        },
        Pet {
            pet_type: "Dog".to_string(),
            name: "Rex".to_string(),
            age: "7".to_string(),
        },
    ];

    // Specify the output file path
    let txt_file_path = Path::new("data/output.txt");
    let txt_file = File::create(txt_file_path)?;
    let mut txt_writer = BufWriter::new(txt_file);

    // Write header "Type Name Age" to the file
    writeln!(txt_writer, "Type Name Age")?; // Header

    // Write each pet's details, space-separated
    for pet in pets {
        // use pet fields separated by space and write it to file
        let pet_str = format!("{} {} {}", pet.pet_type, pet.name, pet.age);
        writeln!(txt_writer, "{}", pet_str)?;
    }

    // Ensure all data is written to the file and print a confirmation message
    txt_writer.flush()?;

    let file_content = std::fs::read_to_string(txt_file_path)?;
    println!("File content:");
    println!("{}", file_content);

    Ok(())
}
