#![allow(non_snake_case)]
use std::{env, path::Path};

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
        println!("{:?}", file);
    }
}
