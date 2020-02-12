use std::env;
use std::fs;

use es_data_parser::validate;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args[1].clone();

    println!("reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let parsed = validate(&contents);
    if let Ok(parsed) = parsed {
        println!("{:#?}", parsed.1);
        std::process::exit(0);
    } else {
        println!("{:#?}", parsed);
        std::process::exit(1);
    }
}
