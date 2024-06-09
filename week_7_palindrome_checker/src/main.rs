use std::io;

fn main() {
    // Instantiates the input String
    let mut input:String = String::new();

    // Friendly text
    println!("Enter a string to see if it's palindrome!");

    // Sets the value of input and checks for errors
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to parse String");

    // Trims any tailing spaces and such
    input = input.trim().to_string().to_lowercase();

    // Version of input with no spaces
    let clean_input:String = input.chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    // Instantiates the reversed_input String for the reversed input
    let reversed_input:String = clean_input.chars().rev().collect();

    // Checks if the reversed_input is equal to the clean_input
    if reversed_input == clean_input {
        println!("{} is palindrome!", input);
    } else {
        println!("{} is not palindrome", input);
    }
}
