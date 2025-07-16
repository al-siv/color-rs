//! CSV Data Loader
//!
//! Utilities for loading color data from CSV files

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use std::fs::File;
use std::path::Path;

/// Color data entry from CSV
#[derive(Debug, Clone)]
pub struct CsvColorEntry {
    pub code: String,
    pub name: String,
    pub hex: String,
}

/// CSV Color Data Loader
pub struct CsvLoader;

impl CsvLoader {
    /// Load color data from CSV file
    /// Expected format: Code;Name;Hex (semicolon-separated)
    pub fn load_colors_from_csv<P: AsRef<Path>>(file_path: P) -> Result<Vec<CsvColorEntry>> {
        let file = File::open(&file_path).with_context(|| {
            format!("Failed to open CSV file: {}", file_path.as_ref().display())
        })?;

        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .from_reader(file);

        let mut colors = Vec::new();

        for result in reader.records() {
            let record = result.with_context(|| {
                format!(
                    "Failed to read CSV record from {}",
                    file_path.as_ref().display()
                )
            })?;

            if record.len() != 3 {
                anyhow::bail!(
                    "Invalid CSV format: expected 3 columns (Code;Name;Hex), got {}",
                    record.len()
                );
            }

            let entry = CsvColorEntry {
                code: record[0].trim().to_string(),
                name: record[1].trim().to_string(),
                hex: record[2].trim().to_string(),
            };

            // Validate hex format
            if !entry.hex.starts_with('#') || entry.hex.len() != 7 {
                anyhow::bail!("Invalid hex color format: {} (expected #RRGGBB)", entry.hex);
            }

            colors.push(entry);
        }

        if colors.is_empty() {
            anyhow::bail!(
                "No color data found in CSV file: {}",
                file_path.as_ref().display()
            );
        }

        Ok(colors)
    }

    /// Convert hex string to RGB array
    pub fn hex_to_rgb(hex: &str) -> Result<[u8; 3]> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color format: #{} (expected #RRGGBB)", hex);
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .with_context(|| format!("Invalid red component in hex color: #{}", hex))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .with_context(|| format!("Invalid green component in hex color: #{}", hex))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .with_context(|| format!("Invalid blue component in hex color: #{}", hex))?;

        Ok([r, g, b])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_csv_loading() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Code;Name;Hex").unwrap();
        writeln!(temp_file, "black;Black;#000000").unwrap();
        writeln!(temp_file, "white;White;#ffffff").unwrap();

        let colors = CsvLoader::load_colors_from_csv(temp_file.path()).unwrap();

        assert_eq!(colors.len(), 2);
        assert_eq!(colors[0].code, "black");
        assert_eq!(colors[0].name, "Black");
        assert_eq!(colors[0].hex, "#000000");
        assert_eq!(colors[1].code, "white");
        assert_eq!(colors[1].name, "White");
        assert_eq!(colors[1].hex, "#ffffff");
    }

    #[test]
    fn test_invalid_hex_format() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Code;Name;Hex").unwrap();
        writeln!(temp_file, "invalid;Invalid;000000").unwrap(); // Missing #

        let result = CsvLoader::load_colors_from_csv(temp_file.path());
        assert!(result.is_err());
    }
}
