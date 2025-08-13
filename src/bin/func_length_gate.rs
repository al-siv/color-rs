use std::fs;
use std::path::{Path, PathBuf};

const MAX_FN_LOC: usize = 60;

fn main() {
    let src_root = Path::new("src");
    let mut failures: Vec<(String, usize)> = Vec::new();
    let mut scanned = 0usize;
    for entry in walk(src_root) {
        if let Some(ext) = entry.extension() { if ext != "rs" { continue; } }
        let path_str = entry.to_string_lossy().to_string();
        if path_str.contains("/bin/") { continue; }
        let Ok(content) = fs::read_to_string(&entry) else { continue };
        let mut lines = content.lines().enumerate().peekable();
        while let Some((i, line)) = lines.next() {
            if line.trim_start().starts_with("fn ") || line.contains(" fn ") {
                if line.trim_start().starts_with("//") { continue; }
                let mut brace_depth = 0isize;
                let mut body_lines = 0usize;
                let mut signature_seen_open = false;
                if let Some(idx) = line.find('{') {
                    signature_seen_open = true;
                    brace_depth += 1;
                    if line[idx+1..].contains('}') { brace_depth -= 1; }
                }
                while let Some(&(_j, next_line)) = lines.peek() {
                    if !signature_seen_open {
                        if let Some(idx) = next_line.find('{') {
                            signature_seen_open = true;
                            brace_depth += 1;
                            if next_line[idx+1..].contains('}') { brace_depth -= 1; }
                        }
                        lines.next();
                        continue;
                    }
                    body_lines += 1;
                    let opens = next_line.matches('{').count();
                    let closes = next_line.matches('}').count();
                    brace_depth += opens as isize - closes as isize;
                    lines.next();
                    if brace_depth <= 0 { break; }
                }
                if body_lines > MAX_FN_LOC {
                    failures.push((format!("{}:{}", path_str, i+1), body_lines));
                }
                scanned += 1;
            }
        }
    }
    if failures.is_empty() {
        println!("Function length gate PASS (scanned {} fns)", scanned);
    } else {
        eprintln!("Function length gate FAIL: {} functions exceed {} LOC", failures.len(), MAX_FN_LOC);
        for (loc, sz) in &failures { eprintln!(" - {} => {} LOC", loc, sz); }
        std::process::exit(1);
    }
}

fn walk(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for e in entries.flatten() {
            let path = e.path();
            if path.is_dir() {
                if path.file_name().and_then(|s| s.to_str()) == Some("target") { continue; }
                out.extend(walk(&path));
            } else {
                out.push(path);
            }
        }
    }
    out
}
