use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Lightweight CSV viewer in Rust with clean modular design, alignment preview, and customizable separators")]
pub struct Args {
    #[arg(value_name = "file", help = "File to preview the content")]
    pub file: PathBuf,

    #[arg(short = 'd', long, default_value = ",", help = "Set a custom delimiter")]
    pub delimiter: char,

    #[arg(short = 'r', long, default_value = "30", help = "Set number max of rows to render")]
    pub rows: usize,

    #[arg(short = 'n', long, help = "Specify if CSV does not have a header row")]
    pub no_header: bool
}
