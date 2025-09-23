use csv::Reader;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct Car {
    model: String,
    price: f64,
    transmission: String,
    year: i32,
    distance_traveled: i32,
    color: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create sample data files if they don't exist
    create_sample_data_files()?;

    let filenames = ["data_part1.csv", "data_part2.csv", "data_part3.csv"];
    let mut car_data = Vec::new();

    // TODO: Read and process each CSV file
    for filename in &filenames {
        let file_path = Path::new(filename);
        match File::open(file_path) {
            Ok(file) => {
                let mut reader = Reader::from_reader(file);

                // Skip the header
                //reader.seek(csv::Position::new())?;

                // Process each record
                for result in reader.records() {
                    let record = result?;

                    //println!("> {:?}", record);

                    // Extract and parse the fields
                    let model = match record.get(0) {
                        Some(val) => val.trim().to_string(),
                        None => {
                            println!("Missing model in record from {}", filename);
                            continue;
                        }
                    };
                    let price = match record.get(1) {
                        Some(val) => match val.trim().parse::<f64>() {
                            Ok(price) => price,
                            Err(err) => {
                                println!("Error parsing price in {}: {}", filename, err);
                                continue;
                            }
                        },
                        None => {
                            println!("Missing price in record from {}", filename);
                            continue;
                        }
                    };
                    let transmission = match record.get(2) {
                        Some(val) => val.trim().to_string(),
                        None => {
                            println!("Missing transmission in record from {}", filename);
                            continue;
                        }
                    };
                    let year = match record.get(3) {
                        Some(val) => match val.trim().parse::<i32>() {
                            Ok(year) => year,
                            Err(err) => {
                                println!("Error parsing year in {}: {}", filename, err);
                                continue;
                            }
                        },
                        None => {
                            println!("Missing year in record from {}", filename);
                            continue;
                        }
                    };
                    let distance_traveled = match record.get(4) {
                        Some(val) => match val.trim().parse::<i32>() {
                            Ok(dist) => dist,
                            Err(err) => {
                                println!(
                                    "Error parsing distance_traveled in {}: {}",
                                    filename, err
                                );
                                continue;
                            }
                        },
                        None => {
                            println!("Missing distance_traveled in record from {}", filename);
                            continue;
                        }
                    };
                    let color = match record.get(5) {
                        Some(val) => val.trim().to_string(),
                        None => {
                            println!("Missing color in record from {}", filename);
                            continue;
                        }
                    };

                    // Create a Car struct with all the attributes
                    let car = Car {
                        model,
                        price,
                        transmission,
                        year,
                        distance_traveled,
                        color,
                    };

                    // Append it to the car_data vector
                    car_data.push(car);
                }
            }
            Err(err) => {
                println!("Error opening file {}: {}", filename, err);
                continue;
            }
        }
    }

    // Check if car data is not empty before finding the lowest cost car
    if let Some(lowest_cost_car) = car_data
        .iter()
        .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
    {
        println!("Model: {}", lowest_cost_car.model);
        println!("Price: ${:.2}", lowest_cost_car.price);
        println!("Transmission: {}", lowest_cost_car.transmission);
        println!("Year: {:.2}", lowest_cost_car.year);
        println!("Distance Travelled: {}", lowest_cost_car.distance_traveled);
        println!("Color: {:.2}", lowest_cost_car.color);
    } else {
        println!("No valid car data available.");
    }

    Ok(())
}

// Function to create sample data files for demonstration
fn create_sample_data_files() -> Result<(), Box<dyn Error>> {
    // Sample data for part1
    let part1_data = "Model, Price, Transmission, Year, Distance Traveled, Color\n\
                      Toyota Corolla, 19500.00, Automatic, 2018, 30000, Red\n\
                      Honda Civic, 21000.50, Manual, 2019, 25000, Blue\n\
                      Ford Focus, 18750.75, Automatic, 2017, 40000, Black";

    // Sample data for part2
    let part2_data = "Model, Price, Transmission, Year, Distance Traveled, Color\n\
                      BMW 3 Series, 42000.00, Automatic, 2020, 15000, White\n\
                      Mercedes C-Class, 43500.25, Automatic, 2021, 10000, Silver\n\
                      Audi A4, 39999.99, Manual, 2019, 20000, Gray";

    // Sample data for part3
    let part3_data = "Model, Price, Transmission, Year, Distance Traveled, Color\n\
                      Hyundai Elantra, 17250.50, Automatic, 2016, 50000, Blue\n\
                      Kia Forte, 16800.00, Manual, 2015, 60000, Red\n\
                      Mazda 3, 19200.75, Automatic, 2018, 35000, Black";

    // Write data to files
    std::fs::write("data_part1.csv", part1_data)?;
    std::fs::write("data_part2.csv", part2_data)?;
    std::fs::write("data_part3.csv", part3_data)?;

    Ok(())
}
