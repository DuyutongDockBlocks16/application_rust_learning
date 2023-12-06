use std::fs::read_dir;
fn list_dir_entry_paths(path: &str)-> std::io::Result<()>{
    for entry in read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let filename = entry.file_name();
        if metadata.is_dir() == false{
            println!("{}/{}", path, filename.to_string_lossy())
        } else {
            println!("{}/{}/", path, filename.to_string_lossy())
        }
    }

    Ok(())
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: find PATH");
        std::process::exit(1);
    }
    let path = &args[1];
    list_dir_entry_paths(path)?;
    Ok(())
}