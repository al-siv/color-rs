use std::io::{Write, Result as IoResult};
use colored::*;

pub struct OutputUtils;

impl OutputUtils {
    /// Writes a formatted header with a specific width and style.
    pub fn write_header<W: Write>(
        output: &mut W,
        header: &str,
        width: usize,
    ) -> IoResult<()> {
        let header_text = header.to_uppercase();
        let padding = width.saturating_sub(header_text.len());
        
        writeln!(
            output,
            "{}{:#<padding$}",
            header_text.bold().cyan(),
            "",
            padding = padding
        )
    }

    /// Writes a label and value with alignment and color.
    pub fn write_label_value<W: Write>(
        output: &mut W,
        label: &str,
        value: &str,
        width: usize,
    ) -> IoResult<()> {
        writeln!(
            output,
            "{:>width$}: {}",
            label.bold().green(),
            value.bright_white(),
            width = width
        )
    }

    /// Writes an error message with red color.
    pub fn write_error<W: Write>(
        output: &mut W,
        message: &str,
    ) -> IoResult<()> {
        writeln!(output, "{}: {}", "Error".bold().red(), message)
    }

    /// Writes a success message with green color.
    pub fn write_success<W: Write>(
        output: &mut W,
        message: &str,
    ) -> IoResult<()> {
        writeln!(output, "{}: {}", "Success".bold().green(), message)
    }

    /// Writes a warning message with yellow color.
    pub fn write_warning<W: Write>(
        output: &mut W,
        message: &str,
    ) -> IoResult<()> {
        writeln!(output, "{}: {}", "Warning".bold().yellow(), message)
    }

    /// Writes a separator line.
    pub fn write_separator<W: Write>(
        output: &mut W,
        width: usize,
    ) -> IoResult<()> {
        writeln!(output, "{:-<width$}", "", width = width)
    }
}