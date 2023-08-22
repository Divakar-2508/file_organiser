#![allow(non_snake_case)]
use std::{env,fs};
use std::io::Write;

fn main() {
    let curr_dirr = env::current_dir().expect("Can't read directory");
    
    let files = curr_dirr.read_dir()
        .expect("Cant' take files")
        .filter_map(|entry|{
            let path = entry.unwrap().path();
            if path.is_file(){
                Some(path)
            } else {
                None
            }
        });
    
    for file in files{
        let file_extension = file.extension().unwrap().to_str().unwrap();
        let folder_name = curr_dirr.join(file_extension);

        if !folder_name.exists(){
            std::fs::create_dir(&folder_name).expect("Can't create folder");
        } 

        let new_path = folder_name.join(file.file_name().unwrap());
        fs::rename(&file, &new_path).expect("Can't rename file");
        println!("Files Organized!!");
        std::io::stdout().flush().unwrap();
    }
}
