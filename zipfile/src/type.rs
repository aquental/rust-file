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

        // TODO: Determine if the entry is a directory
        // - Check if entry.name() ends with '/'
        if entry.name().ends_with('/') {
            println!("{} is a directory", entry.name());
        } else {
            println!("{} is a file", entry.name());
        }
    }

    Ok(())
}
