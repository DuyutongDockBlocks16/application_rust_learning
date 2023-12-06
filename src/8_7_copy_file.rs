use std::fs;
use std::io::{self, Write};  // Import the Write trait

fn copy(source: &str, dest: &str) -> io::Result<()> {
    let source_content = fs::read(source)?;

    // Open the destination file and write the content
    let mut dest_file = fs::File::create(dest)?;
    dest_file.write_all(&source_content)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cp SOURCE DEST");
        std::process::exit(1);
    }
    let source = &args[1];
    let dest = &args[2];

    copy(source, dest)?;

    Ok(())
}
