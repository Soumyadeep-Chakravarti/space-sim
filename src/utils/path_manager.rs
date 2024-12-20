use dirs::{document_dir, desktop_dir};
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

pub struct PathManager {
    documents_dir: PathBuf,
    desktop_dir: PathBuf,
}

impl PathManager {
    // Constructor that initializes the path manager
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let documents_dir = document_dir().ok_or("Failed to locate Documents directory")?;
        let desktop_dir = desktop_dir().ok_or("Failed to locate Desktop directory")?;

        Ok(PathManager {
            documents_dir,
            desktop_dir,
        })
    }

    // Returns the path to the space-simulator/config directory in Documents
    pub fn get_space_simulator_config_path(&self) -> PathBuf {
        self.documents_dir
            .join("space-simulator")
            .join("config")
    }

    pub fn get_space_simulator_config_path(&self) -> PathBuf {
        self.documents_dir
            .join("space-simulator")
            .join("config")
    }

    // Ensures the directory exists; creates it if necessary
    pub fn ensure_directory_exists(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    // Returns the path to a file on the Desktop
    pub fn get_desktop_file_path(&self, filename: &str) -> PathBuf {
        self.desktop_dir.join(filename)
    }
}
