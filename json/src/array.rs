use serde_json::Value;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the JSON file
    let file_path = Path::new("data/data.json");

    // Read the content of the JSON file into a string
    let content = fs::read_to_string(file_path)?;

    // Parse the JSON content into a Value
    let data: Value = serde_json::from_str(&content)?;

    // Initialize variables to calculate total experience and count of employees
    let mut total_experience = 0;
    let mut employee_count = 0;

    // TODO: Retrieve the "departments" array from the JSON data and iterate through it
    // Hints:
    // - Use a `for` loop on `departments`
    // - Use another `for` loop on `employees` within the department
    if let Some(departments) = data["departments"].as_array() {
        for department in departments {
            let dept_name = department["name"].as_str().unwrap_or("Unknown");
            println!("Department: {}", dept_name);

            // Access the employees array within each department
            if let Some(employees) = department["employees"].as_array() {
                for employee in employees {
                    // Safely extract employee details
                    let name = employee["name"].as_str().unwrap_or("No Name");
                    let position = employee["position"].as_str().unwrap_or("No Position");
                    let experience = employee["experience"].as_u64().unwrap_or(0);
                    total_experience += experience;
                    employee_count += 1;

                    println!("  - {} ({}, {} years)", name, position, experience);
                }
            }
        }
    }

    // Calculate and display the average experience if employees are found
    if employee_count > 0 {
        let average_experience = total_experience as f64 / employee_count as f64;
        println!(
            "Average Employees' Experience: {:.2} years",
            average_experience
        );
    } else {
        println!("No employees found.");
    }

    Ok(())
}
