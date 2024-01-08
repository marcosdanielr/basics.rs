use std::io;

fn convert_to_int(input: &String) -> i32 {
    let x = input.trim().parse::<i32>().unwrap();

    return x;
}

fn main() {
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Error on read first_number");

    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Error on read second_number");

    if convert_to_int(&first_number) > convert_to_int(&second_number) {
        println!(
            "The number {} is greater than {}",
            first_number, second_number
        );
    } else if convert_to_int(&first_number) == convert_to_int(&second_number) {
        println!(
            "The number {} is equal than {}",
            first_number, second_number
        );
    } else {
        println!("The number {} is less than {}", first_number, second_number);
    }
}
