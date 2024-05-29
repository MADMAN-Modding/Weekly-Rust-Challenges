use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a string to see if it's palindrome!");

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to parse String");

    println!("{}", input.chars().count());
}
