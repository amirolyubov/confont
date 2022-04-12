use std::fs;
use std::env;
// use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("error reading file path");
    };

    let loaded_file = fs::read(&args[1]);
    
    // let file = match fs::File::create("./results/test.txt") {
    //     Err(err) => panic!("error creating file: {}", err),
    //     Ok(file) => file
    // };

    for c in loaded_file.iter() {
        for __c in c {
            fs::write("./results/test.txt", format!("{}", __c)).expect("error writing file");
        }
    };
}