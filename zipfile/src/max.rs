use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn Error>> {
    // Update this path to the actual location of your archive.zip file
    const ZIP_FILE_NAME: &str = "archive.zip";
    let zip_path = Path::new(ZIP_FILE_NAME);

    // Open the ZIP archive for reading
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    // Find and process the data.txt file in the archive
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.name().to_lowercase() == "data.txt" {
            println!("Found file: {}", file.name());

            let reader = BufReader::new(file);
            //let mut sum = 0;
            let mut max = -1;

            // Read and process file line by line
            for line in reader.lines() {
                let line = line?;
                for num_str in line.split_whitespace() {
                    if let Ok(value) = num_str.parse::<i32>() {
                        //println!(" num: {}", value);
                        //sum += value;
                        if value > max {
                            //println!("Max-> {}", value);
                            max = value;
                        }
                    }
                }
            }

            println!("Maximum number in data.txt: {}", max);
            break;
        }
    }

    Ok(())
}
