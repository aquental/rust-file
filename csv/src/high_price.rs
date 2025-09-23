use csv::Reader;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct Car {
    model: String,
    price: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create sample data files if they don't exist
    create_sample_data_files()?;

    let filenames = [
        "data/data_part1.csv",
        "data/data_part2.csv",
        "data/data_part3.csv",
    ];
    let mut car_data = Vec::new();

    for filename in &filenames {
        let file_path = Path::new(filename);
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = Reader::from_reader(file);

                // Process each record
                for result in reader.records() {
                    match result {
                        Ok(record) => {
                            if record.len() >= 2 {
                                if let Ok(price) = record[1].trim().parse::<f64>() {
                                    car_data.push(Car {
                                        model: record[0].trim().to_string(),
                                        price,
                                    });
                                }
                            }
                        }
                        Err(e) => println!("Error reading record: {}", e),
                    }
                }
            }
            Err(err) => {
                println!("Error opening file {}: {}", filename, err);
                continue;
            }
        }
    }

    // Fnd the car with the highest price
    if let Some(highest_cost_car) = car_data
        .iter()
        .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
    {
        // TODO: Display the car with the highest price
        println!("Model: {}", highest_cost_car.model);
        println!("Price: ${:.2}", highest_cost_car.price);
    } else {
        println!("No valid car data available.");
    }

    Ok(())
}

// Function to create sample data files for demonstration
fn create_sample_data_files() -> Result<(), Box<dyn Error>> {
    // Sample data for part1
    let part1_data = "Model, Price\n\
                      Toyota Corolla, 19500.00\n\
                      Honda Civic, 21000.50\n\
                      Ford Focus, 18750.75";

    // Sample data for part2
    let part2_data = "Model, Price\n\
                      BMW 3 Series, 42000.00\n\
                      Mercedes C-Class, 43500.25\n\
                      Audi A4, 39999.99";

    // Sample data for part3
    let part3_data = "Model, Price\n\
                      Hyundai Elantra, 17250.50\n\
                      Kia Forte, 16800.00\n\
                      Mazda 3, 19200.75";

    // Write data to files
    std::fs::write("data/data_part1.csv", part1_data)?;
    std::fs::write("data/data_part2.csv", part2_data)?;
    std::fs::write("data/data_part3.csv", part3_data)?;

    Ok(())
}
