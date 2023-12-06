use std::fs;
use std::io::{self, Write};  // Import the Write trait

fn mv(source: &str, dest: &str) -> io::Result<()> {
    let source_content = fs::read(source)?;

    // Open the destination file and write the content
    let mut dest_file = fs::File::create(dest)?;
    dest_file.write_all(&source_content)?;

    fs::remove_file(source).expect("Unable to remove file");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: mv SOURCE DEST");
        std::process::exit(1);
    }
    let source = &args[1];
    let dest = &args[2];
    mv(source, dest)?;
    Ok(())
}
