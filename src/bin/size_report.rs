//! Module/file size report generator. Outputs Markdown table of top modules by LOC.
use std::fs;
use std::path::Path;

fn main() {
    let root = Path::new("src");
    let mut entries: Vec<(String, usize)> = Vec::new();
    collect(root, root, &mut entries);
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    println!("| Module | LOC |\n|--------|-----|");
    for (path, loc) in entries.iter().take(50) { println!("| {path} | {loc} |"); }
}

fn collect(root: &Path, path: &Path, out: &mut Vec<(String, usize)>) {
    if path.is_dir() {
        if let Ok(rd) = fs::read_dir(path) {
            for entry in rd.filter_map(Result::ok) {
                collect(root, &entry.path(), out);
            }
        }
    } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
        if let Ok(content) = fs::read_to_string(path) {
            let loc = content.lines().count();
            let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().to_string();
            out.push((rel, loc));
        }
    }
}
