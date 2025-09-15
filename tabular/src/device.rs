use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Device {
    name: String,
    power_usage: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the input file path
    let file_path = Path::new("data/devices.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Get an iterator of lines
    let mut lines = reader.lines();

    // Skip the header line
    lines.next();

    // Initialize variables to keep track of the highest power usage and device name
    let mut highest_power_usage = 0;
    let mut highest_power_device_name = String::new();
    let mut devices = Vec::new();

    // Iterate over each line
    for (line_number, line) in lines.enumerate() {
        let line = line?;
        // Split each line into parts by tabs
        let parts: Vec<&str> = line.split('\t').collect();

        // Parse the name and power usage
        if parts.len() >= 2 {
            let power_usage = parts[1].parse::<i32>().map_err(|e| {
                format!(
                    "Failed to parse power usage in line {}: {}",
                    line_number + 2,
                    e
                )
            })?;

            // Create a new Device object and store it
            devices.push(Device {
                name: parts[0].to_string(),
                power_usage,
            });
        } else {
            return Err(format!(
                "Invalid data format in line {}: {:?}",
                line_number + 2,
                line
            )
            .into());
        }
    }

    // Output the highest power usage and the device name
    for device in &devices {
        if device.power_usage > highest_power_usage {
            highest_power_usage = device.power_usage;
            highest_power_device_name = device.name.clone();
        }
    }
    if !highest_power_device_name.is_empty() {
        println!(
            "\nDevice with highest power usage: {} with {} watts",
            highest_power_device_name, highest_power_usage
        );
    } else {
        println!("\nNo valid device data found.");
    }

    Ok(())
}
