use crate::structures;
use dirs::document_dir;
use crate::utils::InputOutput::read_from_json_file;

pub fn validate_credentials() -> structures::dateSaveConfig::Credentials {
    let documents_dir = document_dir().expect("Could not find the system's Documents directory");
    let file_path = documents_dir.join("space-simulator/data/credentials.json");
    let credentials = read_from_json_file::<structures::dateSaveConfig::Credentials>(&file_path).unwrap();
    credentials
}