fn double(number: i32) -> i32 {
    number * 2
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    return b;
}

fn force_type_example(a: f32, b: i128) -> f32 {
    let x = 10.1 * a + b as f32;

    return x;

    // return 10 as f32;
}

fn main() {
    println!("{}", double(2));
    println!("{}", max(1, 2));

    println!("{}", force_type_example(20.0, 1))
}
