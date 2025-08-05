//! Processing pipeline functions
//!
//! Provides functional composition pipelines for preprocessing and postprocessing
//! color input strings. Uses pure functions for transformation chains.

/// Preprocessing step function type
pub type PreprocessingFn = fn(&str) -> String;

/// Post-processing step function type
pub type PostprocessingFn = fn(String) -> String;

/// Preprocessing steps using function composition
#[derive(Debug, Clone)]
pub enum PreprocessingStep {
    /// Normalize whitespace and case
    Normalize,
    /// Trim whitespace
    Trim,
    /// Convert to lowercase
    Lowercase,
    /// Remove special characters
    RemoveSpecialChars,
    /// Custom preprocessing function
    Custom(PreprocessingFn),
}

/// Post-processing steps using function composition
#[derive(Debug, Clone)]
pub enum PostprocessingStep {
    /// Capitalize first letter
    Capitalize,
    /// Title Case conversion
    TitleCase,
    /// Add prefix to output
    AddPrefix(String),
    /// Add suffix to output
    AddSuffix(String),
    /// Custom post-processing function
    Custom(PostprocessingFn),
}

/// Apply preprocessing pipeline using function composition
pub fn apply_preprocessing_pipeline(input: &str, steps: &[PreprocessingStep]) -> String {
    steps.iter().fold(input.to_string(), |acc, step| {
        apply_preprocessing_step(&acc, step)
    })
}

/// Apply a single preprocessing step
pub fn apply_preprocessing_step(input: &str, step: &PreprocessingStep) -> String {
    match step {
        PreprocessingStep::Normalize => normalize_input(input),
        PreprocessingStep::Trim => input.trim().to_string(),
        PreprocessingStep::Lowercase => input.to_lowercase(),
        PreprocessingStep::RemoveSpecialChars => remove_special_chars(input),
        PreprocessingStep::Custom(func) => func(input),
    }
}

/// Apply postprocessing pipeline using function composition
pub fn apply_postprocessing_pipeline(input: String, steps: &[PostprocessingStep]) -> String {
    steps
        .iter()
        .fold(input, |acc, step| apply_postprocessing_step(acc, step))
}

/// Apply a single postprocessing step
pub fn apply_postprocessing_step(input: String, step: &PostprocessingStep) -> String {
    match step {
        PostprocessingStep::Capitalize => capitalize_first(&input),
        PostprocessingStep::TitleCase => to_title_case(&input),
        PostprocessingStep::AddPrefix(prefix) => format!("{}{}", prefix, input),
        PostprocessingStep::AddSuffix(suffix) => format!("{}{}", input, suffix),
        PostprocessingStep::Custom(func) => func(input),
    }
}

/// Normalize whitespace and clean up input
fn normalize_input(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Remove special characters while preserving color format characters
fn remove_special_chars(input: &str) -> String {
    input
        .chars()
        .filter(|c| {
            c.is_alphanumeric()
                || c.is_whitespace()
                || *c == '('
                || *c == ')'
                || *c == ','
                || *c == '#'
        })
        .collect()
}

/// Capitalize first letter of string
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert string to title case
fn to_title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<String>>()
        .join(" ")
}
