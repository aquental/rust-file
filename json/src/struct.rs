use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Employee {
    name: String,
    position: String,
    experience: u64,
}

#[derive(Debug)]
struct Department {
    name: String,
    head: String,
    employees: Vec<Employee>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the JSON file
    let file_path = Path::new("data/data.json");

    // Read the content of the JSON file into a string
    let content = fs::read_to_string(file_path)?;

    // Parse the JSON content into a Value
    let data: Value = serde_json::from_str(&content)?;

    // Step by step, complete the task of mapping JSON to structs
    let departments = data["departments"]
        .as_array()
        .unwrap()
        .iter()
        .map(|department_json| {
            let dept_name = department_json["name"].as_str().unwrap().to_string();
            let dept_head = department_json["head"].as_str().unwrap().to_string();
            let employees = department_json["employees"]
                .as_array()
                .unwrap()
                .iter()
                .map(|employee_json| Employee {
                    name: employee_json["name"].as_str().unwrap().to_string(),
                    position: employee_json["position"].as_str().unwrap().to_string(),
                    experience: employee_json["experience"].as_u64().unwrap(),
                })
                .collect();
            Department {
                name: dept_name,
                head: dept_head,
                employees,
            }
        })
        .collect::<Vec<Department>>();

    // Output the result to verify correctness
    for department in &departments {
        println!("Department: {}, Head: {}", department.name, department.head);
        for employee in &department.employees {
            println!(
                "  Employee: {}, Position: {}, Experience: {} years",
                employee.name, employee.position, employee.experience
            );
        }
    }

    Ok(())
}
