use std::fs;
use std::env;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("error reading file path");
    };

    let loaded_file = fs::read(&args[1]);
    
    let mut file = match fs::File::create("./results/test_woff.txt") {
        Err(err) => panic!("error creating file: {}", err),
        Ok(file) => file
    };

    for c in loaded_file.iter() {
        match file.write_all(c) {
            Err(err) => panic!("error writing to file: {}", err),
            Ok(_) => println!("success writing to file")
        }
    };
}