use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to my guessing game.");

    println!("Pick a number 1-100");


    let mut rng = rand::thread_rng();

    let guessed_number:i8 = rng.gen_range(1..100);

    loop {
        let number = number_picker();

        if number == guessed_number {
            println!("You did it! {} was correct.", guessed_number);
            return;
        } else {
            if number > guessed_number {
                println!("Too high, select another number.");
            } else {
                println!("Too low, select another number.");
            }
        }
    }
}

fn number_picker() -> i8 {
    let mut number:i8 = 0;

    let mut input_number = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("unable to read number");

    let trimmed = input_number.trim();
    match  trimmed.parse::<i8>() {
        Ok(i) => number = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    return number;
}