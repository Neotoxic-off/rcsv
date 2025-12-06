use csv::StringRecord;
use log::warn;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub struct CsvData {
    pub headers: Option<StringRecord>,
    pub rows: Vec<StringRecord>,
    pub column_widths: Vec<usize>,
}

impl CsvData {
    pub fn new() -> Self {
        Self {
            headers: None,
            rows: Vec::new(),
            column_widths: Vec::new(),
        }
    }

    pub fn set_headers(&mut self, headers: StringRecord) {
        self.headers = Some(headers);
    }

    pub fn add_row(&mut self, row: StringRecord) {
        self.rows.push(row);
    }

    fn widen(&mut self, i: usize, field: &str) {
        let width = UnicodeWidthStr::width(field);
        if let Some(w) = self.column_widths.get_mut(i) {
            *w = (*w).max(width);
        }
    }

    pub fn compute_column_widths(&mut self) {
        let cols = self
            .headers
            .as_ref()
            .map(|h| h.len())
            .or_else(|| self.rows.first().map(|r| r.len()))
            .unwrap_or(0);

        if cols == 0 {
            return;
        }

        self.column_widths = vec![0; cols];

        if let Some(h) = self.headers.clone() {
            for (i, f) in h.iter().enumerate() {
                self.widen(i, f);
            }
        }

        let rows_clone = self.rows.clone();
        for r in rows_clone.iter() {
            for (i, f) in r.iter().enumerate() {
                if i < self.column_widths.len() {
                    self.widen(i, f);
                } else {
                    warn!("Row has extra column {}", i);
                }
            }
        }
    }

    pub fn has_data(&self) -> bool {
        !self.rows.is_empty()
    }
}
