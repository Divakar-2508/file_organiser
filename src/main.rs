#![allow(non_snake_case)]
use std::io::{self};
use std::path::PathBuf;
use std::{env, fs};

fn organize_directory(dir: PathBuf) -> io::Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?.path();
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            continue;
        }

        let ext = entry
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or("misc".to_string());

        let ext_folder = dir.join(&ext);
        if !ext_folder.exists() {
            fs::create_dir(&ext_folder)?;
        }

        let new_path = ext_folder.join(entry.file_name().unwrap());

        fs::rename(entry.as_path(), new_path)?;
    }

    Ok(())
}

// fog .
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("\nNo Directory is specified!");
        println!("Usage: fog <dir> (Use '.' for current directory)");
        println!("Example:\nfog .");
        return;
    }

    let dir = PathBuf::from(args.first().unwrap());

    println!("{:?}", dir);
    if !dir.exists() {
        println!("The Given Directory doesn't exists, please check again!");
        return;
    }

    if let Err(err) = organize_directory(dir) {
        match err.kind() {
            io::ErrorKind::PermissionDenied => {
                println!("Insufficient Permission, Try using a elevated Shell")
            }
            io::ErrorKind::Interrupted => println!(
                "Files in the directory is in use by a active program, try again and closing them"
            ),
            _ => println!("Idk Got an error, {}", err),
        }
    } else {
        println!("Directory Organized Successfully!");
    }
}
