# RCSV

✨ Lightweight CSV viewer in Rust with clean modular design, alignment preview, and customizable separators

## Features

* Fast CSV reading with optional headers
* Preview rows in a clean table format
* Customizable delimiter
* Limits displayed rows for large files
* Alternating row colors for readability

## Demo

<img src="assets/preview.gif"/>

## Installation

Make sure you have Rust installed. Clone the repository and build the project:

```bash
git clone https://github.com/Neotoxic-off/rcsv.git
cd rcsv
cargo build --release
```

## Usage

Basic usage:

```bash
cargo run -- path/to/file.csv
```

Optional arguments:

```text
  -d, --delimiter <DELIMITER>  Set a custom delimiter [default: ,]
  -r, --rows <ROWS>            Set number max of rows to render [default: 30]
  -n, --no-header              Specify if CSV does not have a header row
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples

Display a CSV file with default settings:

```bash
cargo run -- ./data/example.csv
```

Display a CSV using a semicolon as delimiter:

```bash
cargo run -- ./data/example.csv -d ";"
```

Display only the first 50 rows:

```bash
cargo run -- ./data/example.csv -r 50
```

Display a CSV file without a header:

```bash
cargo run -- ./data/example.csv --no-header
```

