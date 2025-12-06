use crate::cli::Args;
use log::debug;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CsvConfig {
    pub delimiter: u8,
    pub has_headers: bool,
    pub max_rows: usize,
}

#[derive(Debug)]
pub struct ConfigError {
    message: String,
}

impl ConfigError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration error: {}", self.message)
    }
}

impl Error for ConfigError {}

impl CsvConfig {
    pub fn new(delimiter: char, has_headers: bool, max_rows: usize) -> Result<Self, Box<dyn Error>> {
        debug!("Creating config: delim={}, headers={}, rows={}", delimiter, has_headers, max_rows);
        if !delimiter.is_ascii() {
            return Err(Box::new(ConfigError::new(format!(
                "Delimiter must be ASCII, got {}",
                delimiter
            ))));
        }

        Ok(Self {
            delimiter: delimiter as u8,
            has_headers,
            max_rows,
        })
    }

    fn validate_rows(n: usize) -> Result<(), Box<dyn Error>> {
        if n == 0 {
            return Err(Box::new(ConfigError::new("Rows must be > 0".into())));
        }
        if n > 10000 {
            return Err(Box::new(ConfigError::new("Rows too large (max 10000)".into())));
        }
        Ok(())
    }

    pub fn from_args(args: &Args) -> Result<Self, Box<dyn Error>> {
        Self::validate_rows(args.rows)?;
        Self::new(args.delimiter, !args.no_header, args.rows)
    }
}
