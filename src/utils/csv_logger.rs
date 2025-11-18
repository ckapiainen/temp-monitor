use anyhow::Result;
use chrono::prelude::*;
use csv::{Error, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvCpuLogEntry {
    pub timestamp: String,
    pub temperature_unit: String,
    pub temperature: f32,
    pub cpu_usage: f32,
    pub power_draw: f32,
}
#[derive(Debug)]
pub struct CsvLogger {
    wtr: Writer<File>,
    pub path: PathBuf,
    pub timestamp: DateTime<Local>,
    write_buffer_size: usize,
    pub write_buffer: Vec<CsvCpuLogEntry>,
    pub graph_data: Vec<CsvCpuLogEntry>, // TODO: For upcoming line graph. THIS IS HERE FOR NOW
}

impl CsvLogger {
    pub fn new(custom_dir_path: Option<&str>) -> Result<Self> {
        let dir = custom_dir_path.unwrap_or("logs");
        fs::create_dir_all(dir)?;
        let date_str = Local::now().format("%d-%m-%Y").to_string();
        let path = PathBuf::from(format!("{}/{}_cpu_logs.csv", dir, date_str));
        let wtr = WriterBuilder::new().delimiter(b';').from_path(&path)?;
        Ok(Self {
            wtr,
            path,
            timestamp: Local::now(),
            write_buffer_size: 1,
            write_buffer: vec![],
            graph_data: vec![],
        })
    }

    pub fn update_path(&mut self, new_path: PathBuf) {
        self.path = new_path;
        self.wtr = WriterBuilder::new()
            .delimiter(b';')
            .from_path(&self.path)
            .unwrap();
    }
    pub fn read(&self) -> Result<Vec<CsvCpuLogEntry>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_path(&self.path)?;
        let mut result = vec![];
        for data in rdr.deserialize() {
            let record: CsvCpuLogEntry = data?;
            println!("{:?}", record);
            result.push(record);
        }
        Ok(result)
    }
    pub fn write(&mut self, mut entries: Vec<CsvCpuLogEntry>) -> Result<(), Error> {
        // Check current day if new writer with updated path is needed
        let today = Local::now();
        let date_str = today.format("%d-%m-%Y").to_string();

        if date_str != self.timestamp.format("%d-%m-%Y").to_string() {
            // Flush pending writes before rotating to new file
            self.flush_buffer()?;

            self.timestamp = today;
            let new_filename = format!("logs/{}_cpu_logs.csv", date_str);
            self.path = PathBuf::from(&new_filename);
            self.wtr = WriterBuilder::new().delimiter(b';').from_path(&self.path)?;
        }

        // Add to graph data (last 1000 for now)
        self.graph_data.append(&mut entries.clone());
        if self.graph_data.len() > 1000 {
            self.graph_data.drain(0..self.graph_data.len() - 1000);
        }

        // Add to write buffer
        self.write_buffer.append(&mut entries);
        // Flush at max buffer size
        if self.write_buffer.len() >= self.write_buffer_size {
            self.flush_buffer()?;
        }

        Ok(())
    }

    pub fn flush_buffer(&mut self) -> Result<(), Error> {
        // Check if file still exists, recreate if deleted
        if !self.path.exists() {
            eprintln!("CSV file was deleted, recreating: {:?}", self.path);
            // Ensure parent directory exists
            if let Some(parent) = self.path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    Error::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to create directory: {}", e),
                    ))
                })?;
            }
            // Recreate the writer with the same path
            self.wtr = WriterBuilder::new().delimiter(b';').from_path(&self.path)?;
        }

        for entry in &self.write_buffer {
            self.wtr.serialize(entry)?;
        }
        self.wtr.flush()?;
        self.write_buffer.clear(); // Clear after writing to avoid duplicates
        Ok(())
    }

    pub fn get_graph_data(&self) -> &[CsvCpuLogEntry] {
        &self.graph_data
    }
}
