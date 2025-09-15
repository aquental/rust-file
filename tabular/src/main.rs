
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Person {
    name: String,
    age: i32,
    occupation: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the path to the data file
    let file_path = Path::new("data/data.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Get an iterator of lines
    let lines = reader.lines();

    let mut people = Vec::new();
    for line in lines {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let age = parts[1].parse::<i32>().unwrap_or(0);
            people.push(Person {
                name: parts[0].to_string(),
                age,
                occupation: parts[2].to_string(),
            });
        }
    }

    // Output the parsed people data
    println!("Parsed People Data:");
    for person in &people {
        println!("Name: {}, Age: {}, Occupation: {}", 
                 person.name, person.age, person.occupation);
    }

    Ok(())
}
