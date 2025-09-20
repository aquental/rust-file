use std::error::Error;
use std::fs::File;
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn Error>> {
    // Path of the ZIP file to be read
    let zip_file_path = "archive.zip";

    // Open the ZIP archive for reading
    let file = File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;

    // Iterate over each entry in the ZIP file
    for i in 0..archive.len() {
        let entry = archive.by_index(i)?;

        // Display the name of the file inside the ZIP archive
        println!("File Name: {}", entry.name());

        // TODO: Print the file size using the size() method
        println!("File Size: {}", entry.size());
    }

    Ok(())
}
