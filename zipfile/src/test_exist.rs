use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn Error>> {
    // Define the path of the ZIP file
    const ZIP_FILE_NAME: &str = "archive.zip";
    let zip_path = Path::new(ZIP_FILE_NAME);

    // Open the ZIP archive for reading
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut test_exists = false;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        // TODO: Get the entry for the file we want to read, "test.txt"
        if file.name().to_lowercase() == "test.txt" {
            println!("Found file: {}", file.name());
            test_exists = true;
            // - Open "test.txt" using a BufReader
            let reader = BufReader::new(file);
            // - Read and print the contents to the console
            for line in reader.lines() {
                let line = line?;
                println!("line: {}", line);
            }
        }
        if !test_exists {
            // If it doesn't, print "File test.txt not found in the ZIP archive."
            println!("File test.txt not found in the ZIP archive.");
        }
    }

    Ok(())
}
