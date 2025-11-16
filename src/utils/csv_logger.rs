use anyhow::Result;
use csv::{Error, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvCpuLogEntry {
    pub timestamp: String,
    pub temperature: f32,
    pub cpu_usage: f32,
    pub power_draw: f32,
}
#[derive(Debug)]
pub struct CsvLogger {
    wtr: Writer<File>,
    path: PathBuf,
}

impl CsvLogger {
    pub fn new() -> Result<Self> {
        fs::create_dir_all("logs")?;
        let path = PathBuf::from("logs/cpu_logs.csv");
        let wtr = WriterBuilder::new().delimiter(b';').from_path(&path)?;

        Ok(Self { wtr, path })
    }

    pub fn update_path(&mut self, new_path: PathBuf) {
        self.path = new_path;
    }
    pub fn read(&self) -> Result<Vec<CsvCpuLogEntry>> {
        let mut rdr = csv::Reader::from_path(&self.path)?;
        let mut result = vec![];
        for data in rdr.deserialize() {
            let record: CsvCpuLogEntry = data?;
            println!("{:?}", record);
            result.push(record);
        }
        Ok(result)
    }
    pub fn write(&mut self, entries: Vec<CsvCpuLogEntry>) -> Result<(), Error> {
        // Buffered writer
        for entry in entries {
            self.wtr.serialize(entry)?;
        }
        self.wtr.flush()?;
        Ok(())
    }
}
