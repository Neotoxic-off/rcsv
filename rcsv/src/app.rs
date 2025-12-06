use crate::domain::{config::CsvConfig, data::CsvData};
use crate::services::{formatter::CsvFormatter, reader::CsvReader};
use log::{error, warn};
use std::error::Error;
use std::path::PathBuf;

pub struct CsvViewerApp {
    reader: Box<dyn CsvReader>,
    formatter: Box<dyn CsvFormatter>,
}

impl CsvViewerApp {
    pub fn new(reader: Box<dyn CsvReader>, formatter: Box<dyn CsvFormatter>) -> Self {
        Self { reader, formatter }
    }

    pub fn run(&self, path: &PathBuf, config: &CsvConfig) -> Result<(), Box<dyn Error>> {
        let data = self.load_data(path, config)?;
        self.handle_empty(&data)?;
        self.display(&data);
        Ok(())
    }

    fn load_data(&self, path: &PathBuf, config: &CsvConfig) -> Result<CsvData, Box<dyn Error>> {
        match self.reader.read(path, config) {
            Ok(d) => Ok(d),
            Err(e) => {
                error!("Failed to read CSV: {}", e);
                Err(e)
            }
        }
    }

    fn handle_empty(&self, data: &CsvData) -> Result<(), Box<dyn Error>> {
        if data.has_data() {
            return Ok(());
        }

        match &data.headers {
            Some(_) => {
                warn!("CSV has headers but no data rows");
                Ok(())
            }
            None => {
                error!("CSV file is empty");
                Err("CSV file contains no data".into())
            }
        }
    }

    fn display(&self, data: &CsvData) {
        if let Some(output) = self.formatter.format(data) {
            println!("\n{}\n", output);
        }
    }
}
