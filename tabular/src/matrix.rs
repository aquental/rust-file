use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Define a 3x3 matrix of integers
    let matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // Specify the output file path
    let output_file_path = "data/moutput.txt";
    let file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(file);

    // TODO: Prepare data to write to the file
    // - Convert each row to a space-separated string
    // - Write each row to the file with a newline character
    for row in matrix {
        let row_str = row
            .into_iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(writer, "{}", row_str)?;
    }

    writer.flush()?;
    println!("Matrix successfully written to {}.", output_file_path);

    Ok(())
}
