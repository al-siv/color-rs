//! Binary: function length scanning gate. Exits non-zero if any function exceeds threshold.
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
struct FnStat {
    file: PathBuf,
    name: String,
    start: usize,
    end: usize,
    loc: usize,
}

fn scan_functions(root: &Path, max_loc: usize) -> Vec<FnStat> {
    let mut out = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("rs") { continue; }
        let p = path.to_string_lossy();
        if p.contains("/tests/") || p.contains("\\tests\\") || p.contains("/target/") || p.contains("\\target\\") { continue; }
        let Ok(src) = std::fs::read_to_string(path) else { continue };
        let lines: Vec<&str> = src.lines().collect();
        let mut i = 0usize;
        while i < lines.len() {
            if let Some(pos) = lines[i].find("fn ") {
                let after = &lines[i][pos + 3..];
                let name: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                if name.is_empty() { i += 1; continue; }
                let mut depth = lines[i][pos..].matches('{').count() as i32 - lines[i][pos..].matches('}').count() as i32;
                let start = i;
                while depth == 0 && i + 1 < lines.len() {
                    i += 1;
                    let l2 = lines[i];
                    depth += l2.matches('{').count() as i32 - l2.matches('}').count() as i32;
                }
                while depth > 0 && i + 1 < lines.len() {
                    i += 1;
                    let l3 = lines[i];
                    depth += l3.matches('{').count() as i32 - l3.matches('}').count() as i32;
                }
                let end = i;
                let loc = end + 1 - start;
                if loc > max_loc { out.push(FnStat { file: path.to_path_buf(), name, start: start + 1, end: end + 1, loc }); }
            }
            i += 1;
        }
    }
    out
}

fn main() {
    let mut args = env::args().skip(1);
    let root = PathBuf::from(args.next().unwrap_or_else(|| "src".into()));
    let max: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(60);
    let offenders = scan_functions(&root, max);
    if offenders.is_empty() { return; }
    eprintln!("Function length gate failed (>{max} LOC):");
    for o in offenders { eprintln!("{}:{}-{} {} ({} LOC)", o.file.display(), o.start, o.end, o.name, o.loc); }
    std::process::exit(1);
}
