use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Lightweight CSV viewer in Rust with clean modular design, alignment preview, and customizable separators")]
pub struct Args {
    #[arg(value_name = "file")]
    pub file: PathBuf,

    #[arg(short = 'd', long, default_value = ",")]
    pub delimiter: char,

    #[arg(short = 'r', long, default_value = "100")]
    pub rows: usize,

    #[arg(short = 'n', long)]
    pub no_header: bool
}
