use std::io::stdin;

fn main() {
    let mut number:i128 = 0;

    while  number == 0 {

        println!("Input a number to see it's factorial");

        // Read and parse number to i128
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        number = input.trim().parse().unwrap();

        if number > 33 {
            println!("Your number can't be bigger than 33");
            number = 0;
        }
    }

    println!("The factorial of {} is {}", number, factorial(number));
}

fn factorial(number:i128) -> i128 {
    let mut factorial:i128 = 1;
    for i in 1..(number+1) {factorial*=i; }
    return  factorial;
}
