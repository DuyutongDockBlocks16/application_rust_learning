use std::{fs, path::Path};

const INDENT_SIZE: usize = 4;

fn print_indented(text: &str, depth: usize) {
    let indentation = " ".repeat(depth * INDENT_SIZE);
    println!("{}{}", indentation, text);
}

fn eprint_indented(text: &str, depth: usize) {
    let indentation = " ".repeat(depth * INDENT_SIZE);
    eprintln!("{}Error: {}", indentation, text);
}

fn tree(path: &Path, depth: usize) {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            eprint_indented(&err.to_string(), depth);
            return;
        }
    };

    let mut entries: Vec<_> = dir
        .filter_map(|entry| entry.ok())
        .collect();

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let dirname = path
        .file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy();

    print_indented(&dirname, depth);

    for entry in entries {
        if entry.path().is_dir() {
            tree(&entry.path(), depth + 1);
        } else {
            let filename = entry.file_name().to_string_lossy().to_string(); // Create a String
            print_indented(&filename, depth + 1);
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap_or_else(|| ".".to_string());
    tree(Path::new(&path), 0);
}
