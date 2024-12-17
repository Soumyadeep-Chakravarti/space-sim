use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::error::Error;

// Struct representing the CSV rows
#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub id: u32,
    pub name: String,
    pub value: f64,
}

// Function to read a CSV file
pub fn read_csv(filepath: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(filepath)?;
    let mut records: Vec<Record> = Vec::new();
    for result in reader.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

// Function to create a new CSV file and write records to it
pub fn write_csv(filepath: &str, records: &[Record]) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().from_path(filepath)?;
    for record in records {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}

// Function to append records to an existing CSV file
pub fn append_to_csv(filepath: &str, new_data: &[Record]) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().has_headers(false).from_path(filepath)?;
    for record in new_data {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}
