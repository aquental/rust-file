use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    // Reading names from the file
    let file_path = Path::new("names.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut names: Vec<String> = Vec::new();

    // TODO: Implement a loop to read data line-by-line
    for line in reader.lines() {
        let name = line.unwrap();
        // TODO: Add code to store the name in the vector using the push method
        names.push(name);
        // TODO: Add code to sort the vector alphabetically using the sort method
        names.sort();
    }

    // Printing the sorted names
    println!("Sorted names:");
    for name in names {
        println!("{}", name);
    }

    Ok(())
}
