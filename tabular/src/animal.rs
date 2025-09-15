use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Animal {
    name: String,
    habitat: String,
    average_lifespan: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the path to the data file
    let file_path = Path::new("data/animal_data.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Get an iterator of lines
    let mut lines = reader.lines();

    // Skip the header line
    lines.next();

    let mut animals = Vec::new();
    for (line_number, line) in lines.enumerate() {
        let line = line?;
        // Split each line by tabs
        let parts: Vec<&str> = line.split('\t').collect();

        // Parse each line into tokens
        if parts.len() >= 3 {
            let average_lifespan = parts[2].parse::<f64>().map_err(|e| {
                format!(
                    "Failed to parse average_lifespan in line {}: {}",
                    line_number + 2,
                    e
                )
            })?;

            // Create a new Animal object
            animals.push(Animal {
                name: parts[0].to_string(),
                habitat: parts[1].to_string(),
                average_lifespan,
            });
        } else {
            return Err(format!(
                "Invalid data format in line {}: {:?}",
                line_number + 2,
                line
            )
            .into());
        }
    }

    // Output the parsed animal data
    println!("Parsed Animal Data:");
    for animal in &animals {
        println!(
            "Name: {}, Habitat: {}, Average Lifespan: {} years",
            animal.name, animal.habitat, animal.average_lifespan
        );
    }

    Ok(())
}
