use std::io::{self};

fn main() {
    println!("Insert a number to see if its prime");

    let mut number:i16 = 0;

    let mut input_number = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("failed to read from stdin");

    let trimmed = input_number.trim();
    match trimmed.parse::<i16>() {
        Ok(i) => number = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    // The maximum number that it can be divided by to check
    let max_dividend = number / 2;
    
    let mut multiples:Vec<i16> = Vec::new();

    if check_num(number, max_dividend, &mut multiples) {
        println!("It isn't prime :(");

        println!("It's multiples are: ");

        for n in multiples {
            println!("{}", n);
        }
    } else {
        println!("It's prime!")
    }
        
}

fn check_num(number:i16, max_dividend:i16, array_pointer:&mut Vec<i16>) -> bool {
    
    let mut i = 0;

    while i < max_dividend {
        if number % (max_dividend-i) == 0 && max_dividend - i != 1 {
            array_pointer.push(max_dividend-i);
        }
        
        i += 1;
    }

    if !array_pointer.is_empty() {
        return  true;
    }

    return  false;

}
