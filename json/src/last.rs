use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Participant {
    name: String,
    project: String,
}

#[derive(Serialize, Deserialize)]
struct Event {
    name: String,
    date: String,
    participants: Vec<Participant>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a vector of participant structs
    let participants = vec![
        Participant {
            name: "Alex".to_string(),
            project: "Volcano Model".to_string(),
        },
        Participant {
            name: "Jordan".to_string(),
            project: "Robotics".to_string(),
        },
        Participant {
            name: "Taylor".to_string(),
            project: "Solar System".to_string(),
        },
    ];

    // Create an event struct
    let event = Event {
        name: "Science Fair".to_string(),
        date: "2023-05-25".to_string(),
        participants,
    };

    // TODO: Convert the event struct to a JSON object
    let event_json = json!({
        "name": event.name,
        "date": event.date,
        "participants": event.participants.iter().map(|p| {
            json!({
                "name": p.name,
                "project": p.project
            })
        }).collect::<Vec<_>>()
    });

    // TODO: Render the complete event JSON object to a JSON string with indent
    let json_string = serde_json::to_string_pretty(&event_json)?;

    // TODO: Specify the output file path to "event_data.json"
    let dir_path = Path::new("..");
    let output_file_path = dir_path.join("event_data.json");
    let file = File::create(&output_file_path)?;
    let mut writer = BufWriter::new(file);

    // TODO: Write the JSON string to a file
    writer.write_all(json_string.as_bytes())?;

    Ok(())
}
