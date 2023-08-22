#![allow(non_snake_case)]
use std::path::Path;
use std::{ env, fs };
use std::io::Write;

fn main() {
    let enb = env::args().collect::<Vec<String>>();
    let curr_dirr = env::current_dir().expect("Can't read directory");

    let dir;

    if enb.len() > 1 {
        if Path::new(&enb[1]).is_dir() {
            dir = Path::new(&enb[1]);
        } else if enb[1] == "."{
            dir = &curr_dirr; 
        } else {
            println!("Invalid Path");
            println!("Usage: file_organizer [path] (. for current directory)");
            return;
        }
    } else {

        print!("Do you want to Organise the current Directory? (y/n): ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Can't read input");

        if input.trim() == "y" || input.trim() == "Y" {
            dir = &curr_dirr;
        } else {
            println!("Mission Aborted!! :)");
            println!("Usage: file_organizer [path] (. for current directory)");
            return;
        }
    }

    let files = dir
        .read_dir()
        .expect("Cant' take files")
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        });

    for file in files {
        if let Some(extension) = file.extension() {
            let file_extension = extension.to_str().unwrap();
            let folder_name = dir.join(file_extension);

            if !folder_name.exists() {
                std::fs::create_dir(&folder_name).expect("Can't create folder");
            }

            let new_path = folder_name.join(file.file_name().unwrap());
            fs::rename(&file, &new_path).expect("Can't rename file");
        }
    }

    println!("Files Organized!!");
    std::io::stdout().flush().unwrap();
}
