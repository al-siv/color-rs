//! Simple function length gate: scans Rust source files under `src/` (excluding generated or this file) and reports
//! any function whose body exceeds the soft cap (default 60 LOC). Heuristic (regex-based) to avoid heavy parsing.
//! This is a non-fatal tool: exits with code 0 but prints a FAIL summary if violations exist (integration can choose to fail).

use std::{env, fs, path::Path};

use regex::Regex;

const DEFAULT_LIMIT: usize = 60;

fn main() {
	let limit: usize = env::var("FUNC_LEN_LIMIT").ok().and_then(|v| v.parse().ok()).unwrap_or(DEFAULT_LIMIT);
	let src_root = Path::new("src");
	if !src_root.exists() {
		eprintln!("src directory not found");
		return;
	}

	let func_re = Regex::new(r"(?m)^\s*(pub\s+)?(async\s+)?(const\s+)?(unsafe\s+)?fn\s+([a-zA-Z0-9_]+)\s*\(").unwrap();
	let mut violations = Vec::new();

	for entry in walkdir::WalkDir::new(src_root) {
		let entry = match entry { Ok(e) => e, Err(_) => continue };
		if !entry.file_type().is_file() { continue; }
		let path = entry.path();
		if path.extension().and_then(|s| s.to_str()) != Some("rs") { continue; }
		if path.ends_with("func_length_gate.rs") { continue; }
		let content = match fs::read_to_string(path) { Ok(c) => c, Err(_) => continue };

		for cap in func_re.captures_iter(&content) {
			let name = cap.get(5).map(|m| m.as_str()).unwrap_or("<unknown>");
			if let Some(start) = cap.get(0) {
				// naive body slice: find next '{' after signature, then match braces
				if let Some(sig_end) = content[start.end()..].find('{') {
					let body_start_index = start.end() + sig_end + 1; // char after '{'
					let mut depth = 1usize;
					let mut i = body_start_index;
					let bytes = content.as_bytes();
					while i < content.len() && depth > 0 {
						match bytes[i] as char {
							'{' => depth += 1,
							'}' => depth -= 1,
							_ => {}
						}
						i += 1;
					}
					if depth == 0 {
						// compute line span
						let body_slice = &content[body_start_index..i-1];
						let body_lines = body_slice.lines().count();
						if body_lines > limit {
							violations.push((body_lines, name.to_string(), path.display().to_string()));
						}
					}
				}
			}
		}
	}

	if violations.is_empty() {
		println!("Function length gate PASS (limit={limit} lines)");
	} else {
		println!("Function length gate FAIL (limit={limit} lines) — {} violations", violations.len());
		violations.sort_by(|a,b| b.0.cmp(&a.0));
		for (lines, name, path) in &violations {
			println!("{lines:4} lines  {name}  ({path})");
		}
	}
}
