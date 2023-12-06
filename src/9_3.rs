use std::fs;
use std::path::Path;

const INDENT_SIZE: usize = 4;

fn print_indented(text: &str, depth: usize) {
    let indentation = " ".repeat(depth * INDENT_SIZE);
    println!("{indentation}{text}");
}

fn eprint_indented(text: &str, depth: usize) {
    let indentation = " ".repeat(depth * INDENT_SIZE);
    eprintln!("{indentation}Error: {text}");
}

fn print_dir_and_contents(path: &Path, depth: usize) {
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(err) => {
            eprint_indented(&err.to_string(), depth);
            return;
        }
    };

    let dirname = path
        .file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy();

    print_indented(&dirname, depth);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {

                let file_name = entry.file_name();

                print_indented(&file_name.to_string_lossy(), depth+1);
                // println!("{}", file_name.to_string_lossy());
            }
        }
    } else {
        // eprintln!("Failed to read directory: {}", dir_path);
    }
}

fn main() {
    print_dir_and_contents(Path::new("src"),0);
}