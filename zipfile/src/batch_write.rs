use rand::Rng;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const FILE_PATH: &str = "large_data/data.csv";
    const NUM_BATCHES: usize = 5;
    const BATCH_SIZE: usize = 200;
    const NUM_COLUMNS: usize = 10;

    let file_path = Path::new(FILE_PATH);

    // Create directory if it doesn't exist
    if let Some(parent) = file_path.parent() {
        create_dir_all(parent)?;
    }

    // Open the file for writing/appending
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)?;

    let mut writer = BufWriter::new(file);
    let mut rng = rand::thread_rng();

    for batch in 0..NUM_BATCHES {
        for _ in 0..BATCH_SIZE {
            for j in 0..NUM_COLUMNS {
                write!(writer, "{:.2}", rng.r#gen::<f64>() * 100.0)?;
                if j < NUM_COLUMNS - 1 {
                    write!(writer, ", ")?;
                }
            }
            writeln!(writer)?;
        }
        println!("Written batch {} to {}.", batch + 1, file_path.display());
        writer.flush()?;
    }

    // Verify total lines using a streaming approach
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();
    println!("The file {} has {} lines.", file_path.display(), line_count);

    Ok(())
}
