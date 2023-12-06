use std::fs;
use std::io::{self, Write};  // Import the Write trait

fn append_to_file(source: &str, dest: &str) -> io::Result<()> {
    let source_content = fs::read_to_string(source)?;

    // Open the destination file and write the content
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open(dest)
        .expect("Unable to open file");

    write!(file, "{}", source_content).expect("Unable to write data");
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: append SOURCE DEST");
        std::process::exit(1);
    }
    let source = &args[1];
    let dest = &args[2];
    append_to_file(source, dest)?;
    Ok(())
}
