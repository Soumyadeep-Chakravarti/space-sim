use reqwest::blocking::Client;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use csv;
use crate::path_manager::PathManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct HorizonObject {
    pub name: String,
    pub class: String,
    pub ephemeris_id: String,
}

pub fn fetch_observable_objects() -> Result<Vec<HorizonObject>, Box<dyn Error>> {
    // Input file content to query object data
    let input_file_content = r#"
        !$$SOF
        COMMAND= 'MB'
        OBJ_DATA= 'YES'
        MAKE_EPHEM= 'NO'
        !$$EOF
    "#;

    let client = Client::new();
    let form = multipart::Form::new()
        .text("format", "text")
        .text("input", input_file_content);

    // Send the request
    let response = client
        .post("https://ssd.jpl.nasa.gov/api/horizons_file.api")
        .multipart(form)
        .send()?;

    if response.status().is_success() {
        let text = response.text()?;
        let objects = parse_horizon_objects(&text);
        Ok(objects)
    } else {
        Err(Box::new(response.status()))
    }
}

fn parse_horizon_objects(data: &str) -> Vec<HorizonObject> {
    let mut objects = Vec::new();
    for line in data.lines() {
        if let Some(object) = parse_object_line(line) {
            objects.push(object);
        }
    }
    objects
}

fn parse_object_line(line: &str) -> Option<HorizonObject> {
    // Example line format: "199 Mars Planet"
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        Some(HorizonObject {
            ephemeris_id: parts[0].to_string(),
            name: parts[1].to_string(),
            class: parts[2].to_string(),
        })
    } else {
        None
    }
}

pub fn save_to_csv(objects: &[HorizonObject]) -> Result<(), Box<dyn Error>> {
    // Use the PathManager to get the path
    let path_manager = PathManager::new()?;
    let save_path = path_manager.get_space_simulator_data_path();

    // Ensure the directory exists
    path_manager.ensure_directory_exists(&save_path)?;

    // Path to the file to save
    let file_path = save_path.join("horizon_objects.csv");

    // Save the data to CSV
    let file = File::create(file_path)?;
    let mut writer = csv::Writer::from_writer(file);
    for object in objects {
        writer.serialize(object)?;
    }
    writer.flush()?;
    Ok(())
}
