use crate::domain::data::CsvData;
use colored::*;
use csv::StringRecord;
use log::{debug, error, info};

pub trait CsvFormatter {
    fn format(&self, data: &CsvData) -> Option<String>;
}

pub struct TableFormatter;

impl TableFormatter {
    pub fn new() -> Self {
        Self
    }

    fn border(&self, widths: &[usize], left: &str, mid: &str, right: &str) -> String {
        let parts: Vec<String> = widths.iter().map(|w| "─".repeat(w + 2)).collect();
        format!("{}{}{}", left, parts.join(mid), right)
    }

    fn top(&self, widths: &[usize]) -> String {
        self.border(widths, "┌", "┬", "┐")
    }

    fn sep(&self, widths: &[usize]) -> String {
        self.border(widths, "├", "┼", "┤")
    }

    fn bottom(&self, widths: &[usize]) -> String {
        self.border(widths, "└", "┴", "┘")
    }

    fn style_cell(&self, text: String, is_header: bool, is_alt: bool) -> String {
        if is_header {
            text.bright_cyan().bold().to_string()
        } else if is_alt {
            text.bright_white().to_string()
        } else {
            text.white().to_string()
        }
    }

    fn format_row(
        &self,
        record: &StringRecord,
        widths: &[usize],
        is_header: bool,
        is_alt: bool
    ) -> String {
        let mut cells = Vec::new();

        for (i, field) in record.iter().enumerate() {
            let w = *widths.get(i).unwrap_or(&field.len());
            let padded = format!("{:width$}", field, width = w);
            cells.push(self.style_cell(padded, is_header, is_alt));
        }

        format!("│ {} │", cells.join(" │ "))
    }
}

impl CsvFormatter for TableFormatter {
    fn format(&self, data: &CsvData) -> Option<String> {
        debug!("Formatting CSV");

        if data.column_widths.is_empty() {
            error!("No column widths");
            return None;
        }

        let mut out = Vec::new();
        out.push(self.top(&data.column_widths));

        if let Some(h) = &data.headers {
            out.push(self.format_row(h, &data.column_widths, true, false));
            out.push(self.sep(&data.column_widths));
        }

        if data.rows.is_empty() {
            info!("No rows");
        } else {
            for (i, r) in data.rows.iter().enumerate() {
                out.push(self.format_row(r, &data.column_widths, false, i % 2 == 1));
            }
        }

        out.push(self.bottom(&data.column_widths));

        Some(out.join("\n"))
    }
}
