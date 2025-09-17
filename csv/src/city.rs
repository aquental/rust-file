use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file_path = Path::new("data/city_data.csv");
    let file = File::open(file_path)?;

    // Create a CSV reader
    let mut reader = Reader::from_reader(file);

    // Initialize total population and city count
    let mut total_population = 0;
    let mut city_count = 0;

    // Parse each record and calculate total population
    for result in reader.records() {
        let record = result?;
        let population_field = record.get(1).unwrap_or("0");

        // TODO: Convert the value in the population column to an integer
        let population: i32 = population_field.parse().unwrap_or(0);
        // TODO: Add the integer value to total_population
        total_population += population;
        // TODO: Increment the city_count by one per processed row
        city_count += 1;
    }

    if city_count > 0 {
        // TODO: Calculate and display the average population using total_population and city_count
        let average_population = total_population as f64 / city_count as f64;
        println!("Average population: {}", average_population);
    } else {
        println!("No data available.");
    }

    Ok(())
}
