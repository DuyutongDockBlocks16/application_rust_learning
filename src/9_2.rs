use std::fs;
use std::path::Path;

const INDENT_SIZE: usize = 4;

fn print_indented(text: &str, depth: usize) {
    let indentation = " ".repeat(depth * INDENT_SIZE);
    println!("{}{}", indentation, text);
}

fn eprint_indented(text: &str, depth: usize) {
    let indentation = " ".repeat(depth * INDENT_SIZE);
    eprintln!("{}Error: {}", indentation, text);
}

fn print_indented_dirname(path: &Path, depth: usize) {

    if path.is_dir() {
        if let Some(directory_name) = path.file_name() {

            if let Some(name_str) = directory_name.to_str() {
                print_indented(&name_str, depth);
                return
            } else {
                // println!("Failed to convert directory name to string.");
            }
        } else {
            // println!("Path doesn't have a file name.");
        }
    } else {
        eprint_indented("No such file or directory (os error 2)", depth)
    }

}

fn main() {
    print_indented_dirname(Path::new("target/debug"), 1);
    print_indented_dirname(Path::new("nope"), 2);
    print_indented_dirname(Path::new("src/main.rs"), 3);
}
