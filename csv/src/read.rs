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

    for filename in &filenames {
        let file_path = Path::new(filename);

        match File::open(file_path) {
            Ok(file) => {
                let mut reader = Reader::from_reader(file);

                // Process each record
                // Skip the header line
                for result in reader.records() {
                    let record = result?;

                    //println!("Record: {:#?}", record);

                    // Extract and parse the fields
                    let model = record.get(0).ok_or("Missing model")?.to_string();
                    let price = record.get(1).ok_or("Missing price")?.parse::<f64>()?;
                    let transmission = record.get(2).ok_or("Missing transmission")?.to_string();
                    let year = record.get(3).ok_or("Missing year")?.parse::<i32>()?;
                    let distance_traveled =
                        record.get(4).ok_or("Missing distance")?.parse::<i32>()?;
                    let color = record.get(5).ok_or("Missing color")?.to_string();

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
    let part1_data = "Model,Price,Transmission,Year,DistanceTraveled,Color\n\
                      Toyota Corolla,19500.00,Automatic,2018,50000,Red\n\
                      Honda Civic,21000.50,Manual,2019,30000,Blue\n\
                      Ford Focus,18750.75,Automatic,2017,60000,Black";

    // Sample data for part2
    let part2_data = "Model,Price,Transmission,Year,DistanceTraveled,Color\n\
                      BMW 3 Series,42000.00,Automatic,2020,20000,White\n\
                      Mercedes C-Class,43500.25,Manual,2021,15000,Gray\n\
                      Audi A4,39999.99,Automatic,2019,25000,Silver";

    // Sample data for part3
    let part3_data = "Model,Price,Transmission,Year,DistanceTraveled,Color\n\
                      Hyundai Elantra,17250.50,Manual,2016,70000,Red\n\
                      Kia Forte,16800.00,Automatic,2015,80000,Blue\n\
                      Mazda 3,19200.75,Manual,2018,55000,Black";

    // Write data to files
    std::fs::write("data_part1.csv", part1_data)?;
    std::fs::write("data_part2.csv", part2_data)?;
    std::fs::write("data_part3.csv", part3_data)?;

    Ok(())
}
