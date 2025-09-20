use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Participant {
    #[serde(rename = "fullName")]
    name: String,
    project: String,
}

#[derive(Serialize, Deserialize)]
struct Event {
    #[serde(rename = "title")]
    name: String,
    #[serde(rename = "day")]
    date: String,
    participants: Vec<Participant>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let event = Event {
        name: "Science Fair".to_string(),
        date: "2023-05-25".to_string(),
        participants,
    };

    let data = json!(event);

    let dir_path = Path::new("data");
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    let output_file_path = dir_path.join("event.json");
    let file = File::create(&output_file_path)?;
    let mut writer = BufWriter::new(file);

    let json_string = serde_json::to_string_pretty(&data)?;
    writer.write_all(json_string.as_bytes())?;

    println!("Data written to {} as JSON.", output_file_path.display());

    Ok(())
}
