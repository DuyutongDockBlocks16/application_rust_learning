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

    let dirname = path
        .file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy();

    print_indented(&dirname, depth);

    for entry in dir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprint_indented(&err.to_string(), depth + 1);
                continue;
            }
        };

        if entry.file_type().unwrap().is_dir() {
            tree(&entry.path(), depth + 1);
        } else {
            print_indented(entry.file_name().to_string_lossy().as_ref(), depth + 1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        let input_dir = &args[1];
        tree(Path::new(input_dir), 0);
    } else if args.len() == 1 {
        // 如果没有提供目录参数，默认使用当前目录
        tree(Path::new("."), 0);
    } else {
        println!("Wrong number of arguments");
    }
}
