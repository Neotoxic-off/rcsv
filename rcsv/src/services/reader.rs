use crate::domain::{config::CsvConfig, data::CsvData};
use csv::ReaderBuilder;
use log::{debug, error, info, warn};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub trait CsvReader {
    fn read(&self, file: &PathBuf, config: &CsvConfig) -> Result<CsvData, Box<dyn Error>>;
}

pub struct CsvReaderService;

impl CsvReaderService {
    pub fn new() -> Self {
        Self
    }
}

impl CsvReaderService {
    fn open_file(&self, file: &PathBuf) -> Result<BufReader<File>, Box<dyn Error>> {
        let file = match File::open(file) {
            Ok(f) => {
                debug!("File opened");
                f
            }
            Err(e) => {
                error!("Failed to open file {:?}: {}", file, e);
                return Err(Box::new(e));
            }
        };
        Ok(BufReader::new(file))
    }

    fn read_headers(
        &self,
        reader: &mut csv::Reader<BufReader<File>>,
        data: &mut CsvData,
    ) -> Result<(), Box<dyn Error>> {
        match reader.headers() {
            Ok(headers) => {
                data.set_headers(headers.clone());
                Ok(())
            }
            Err(e) => {
                error!("Failed to read headers: {}", e);
                Err(Box::new(e))
            }
        }
    }

    fn read_rows(
        &self,
        reader: &mut csv::Reader<BufReader<File>>,
        data: &mut CsvData,
        max: usize,
    ) -> usize {
        let mut count = 0;

        for result in reader.records() {
            match result {
                Ok(record) => {
                    data.add_row(record);
                    count += 1;
                    if count >= max {
                        info!("Reached max rows limit: {}", max);
                        break;
                    }
                }
                Err(e) => {
                    warn!("Error reading row {}: {}", count, e);
                }
            }
        }

        count
    }
}

impl CsvReader for CsvReaderService {
    fn read(&self, file: &PathBuf, config: &CsvConfig) -> Result<CsvData, Box<dyn Error>> {
        info!("Reading CSV file: {:?}", file);

        let reader = self.open_file(file)?;
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(config.delimiter)
            .has_headers(config.has_headers)
            .from_reader(reader);

        let mut data = CsvData::new();

        if config.has_headers {
            self.read_headers(&mut csv_reader, &mut data)?;
        }

        let count = self.read_rows(&mut csv_reader, &mut data, config.max_rows);

        match count {
            0 => warn!("No rows read"),
            n => info!("Read {} rows", n),
        }

        data.compute_column_widths();
        Ok(data)
    }
}
