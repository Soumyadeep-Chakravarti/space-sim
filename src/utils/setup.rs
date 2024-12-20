use std::fs::create_dir_all;
use std::io::{self};
use serde::Serialize;
use crate::utils::InputOutput::{get_input, save_to_file}; // Import save_to_file
use crate::structures::dateSaveConfig::{Credentials, Database, ApiKeys, Other};
use crate::utils::path_manager::PathManager;

/// Collects user input and returns a `Credentials` object
fn GetData() -> Credentials {
    // Collect user input for the database configuration
    let database = Database {
        host: get_input("Enter database host:"),
        port: get_input("Enter database port:")
            .parse::<u16>()
            .expect("Port must be a valid number"),
        username: get_input("Enter database username:"),
        password: get_input("Enter database password:"),
        database_name: get_input("Enter database name:"),
    };

    // Collect user input for API keys
    let api_keys = ApiKeys {
        nasa_api_key: get_input("Enter NASA API key:"),
        service_2: get_input("Enter Service 2 API key:"),
    };

    // Collect user input for other information
    let other = Other {
        secret_token: get_input("Enter secret token:"),
    };

    // Return the collected credentials
    Credentials {
        database,
        api_keys,
        other,
    }
}

/// Sets up the directory and file, and saves the credentials
pub fn setup() -> Result<(), io::Error> {
    // Use the PathManager to get the path
    let path_manager = PathManager::new()?;
    let save_path = path_manager.get_space_simulator_config_path();

    // Ensure the directory exists
    path_manager.ensure_directory_exists(&save_path)?;

    // Collect credentials data
    let credentials = GetData();

    // Serialize the credentials into JSON format
    let json_data = serde_json::to_string_pretty(&credentials)?;

    // Use the save_to_file function from InputOutput to write the JSON data
    save_to_file(save_path.to_str().unwrap(), &json_data)?;

    println!("Credentials saved to: {:?}", save_path);

    Ok(())
}
