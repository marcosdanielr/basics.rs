use std::{i128, io};

fn convert_to_int(input: &String) -> i128 {
    let result = input.trim().parse::<i128>().unwrap();

    result
}

fn main() {
    let mut factorial_input: String = String::new();
    io::stdin()
        .read_line(&mut factorial_input)
        .expect("Error on read factorial input");

    let mut factorial: i128 = 1;

    let mut factorial_input_int: i128 = convert_to_int(&factorial_input);

    while factorial_input_int > 1 {
        factorial *= factorial_input_int;
        factorial_input_int -= 1;
    }

    println!("The factorial is {}", factorial);
}
