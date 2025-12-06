# CSV Viewer

A lightweight CSV viewer written in Rust, designed for clean modularity, customizable delimiters, and table-formatted output.

## Features

* Fast CSV reading with optional headers
* Preview rows in a clean table format
* Customizable delimiter
* Limits displayed rows for large files
* Alternating row colors for readability

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
-d, --delimiter <char>     Set a custom delimiter (default: ,)
--no-header                Specify if CSV does not have a header row
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
cargo run -- ./data/example.csv -n 50
```

Display a CSV file without a header:

```bash
cargo run -- ./data/example.csv --no-header
```

## Contributing

1. Fork the repository
2. Create a new branch (`git checkout -b feature-name`)
3. Make your changes
4. Submit a pull request

## License

This project is licensed under the MIT License.
