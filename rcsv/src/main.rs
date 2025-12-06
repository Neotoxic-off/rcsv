use std::path::PathBuf;

use clap::Parser;
use log::{error, info};

mod app;
mod cli;
mod domain;
mod services;

use app::CsvViewerApp;
use cli::Args;
use domain::config::CsvConfig;
use services::formatter::TableFormatter;
use services::reader::CsvReaderService;

fn parse_args() -> Args {
    Args::try_parse().unwrap_or_else(|e| e.exit())
}

fn build_config(args: &Args) -> Option<CsvConfig> {
    match CsvConfig::from_args(args) {
        Ok(c) => Some(c),
        Err(e) => {
            error!("Failed to create config: {}", e);
            None
        }
    }
}

fn run_app(file: &PathBuf, config: &CsvConfig) {
    let reader: Box<CsvReaderService> = Box::new(CsvReaderService::new());
    let formatter: Box<TableFormatter> = Box::new(TableFormatter::new());
    let app = CsvViewerApp::new(reader, formatter);

    if let Err(e) = app.run(file, config) {
        error!("Application error: {}", e);
    }
}

fn main() {
    env_logger::init();
    info!("Starting CSV Viewer");

    let args = parse_args();
    let config = match build_config(&args) {
        Some(c) => c,
        None => return,
    };

    run_app(&args.file, &config);
    info!("Application completed successfully");
}
