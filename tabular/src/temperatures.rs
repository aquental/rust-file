use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define path for the temperatures file
    let temperatures_file_path = Path::new("data/temperatures.txt");

    // Read all lines from the temperatures file
    let file = File::open(temperatures_file_path)?;
    let reader = BufReader::new(file);
    let mut celsius_temps: Vec<String> = Vec::new();

    // Convert each temperature from Fahrenheit to Celsius
    for line in reader.lines() {
        let temp_str = line?;
        let fahrenheit: f64 = temp_str.trim().parse()?;
        let celsius = fahrenheit_to_celsius(fahrenheit);
        celsius_temps.push(format!("{:.2}", celsius));
        println!("{}f -> {}c", fahrenheit, celsius);
    }

    // Write the converted temperatures back to the file maintaining new lines
    let mut file = File::create(temperatures_file_path)?;
    for temp in celsius_temps {
        writeln!(file, "{}", temp)?;
    }

    Ok(())
}
