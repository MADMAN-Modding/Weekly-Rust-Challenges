use std::{fs::File, io::{self, stdin, Read}};

fn main() {
    let mut input:String = String::new();

    println!("What file do you want to read?");

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't parse input :(");

    input = input.trim().to_string();

    let mut data_file = File::open(input).unwrap();

    let mut file_content = String::new();

    data_file.read_to_string(&mut file_content).unwrap();

    println!("{}", file_content);
}