use std::io;

fn main() {
    println!("Hi, input how many numbers of the Fibonacci sequence you would like to generate (max is 181): ");

    let mut amount:i16 = 0;

    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("failed to read from stdin");

    let trimmed = input_number.trim();
    match trimmed.parse::<i16>() {
        Ok(i) => amount = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    if amount > 0 && amount < 182 {
        let mut previous_number:i128 = fibonacci(0, 1);

        println!("{}", previous_number);

        let mut current_number:i128 = fibonacci(previous_number, previous_number);

        println!("{}", current_number);

        let mut i:i16 = 0;
        
        while i < amount {
            let temp_num = fibonacci(previous_number, current_number);

            previous_number = current_number;

            current_number = temp_num;

            println!("{}", current_number);

            i += 1;
        }
    } else {
        println!("Number not in range");
        main();
    }
}

fn fibonacci(previous_number:i128, current_number:i128) -> i128 {
    return previous_number + current_number;
}