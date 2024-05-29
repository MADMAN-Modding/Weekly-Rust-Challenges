use std::io;

fn main() {
    print!("Input your temperature in celsius and get it in fahrenheit");

    let mut celsius:i16 = 0;

    let mut input_number = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("unable to read number");

    let trimmed = input_number.trim();
    match  trimmed.parse::<i16>() {
        Ok(i) => celsius = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    println!("{}", convert(celsius));
}

fn convert(celsius: i16) -> i16 {
    return (celsius * (9/5)) + 32;
}