use std::io;

fn convert_to_int(input: &String) -> i32 {
    let result = input.trim().parse::<i32>().unwrap();

    result
}

fn main() {
    let mut sum: i32 = 0;

    let mut input_value: String = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Erro on read input_number");

    let mut input_value_i32: i32 = convert_to_int(&input_value);

    while input_value_i32 != 0 {
        let r = input_value_i32 % 10;
        sum += r;
        input_value_i32 /= 10;
    }

    println!("{}", sum);
}
