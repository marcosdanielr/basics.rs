fn double(number: i32) -> i32 {
    number * 2
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    return b;
}

fn main() {
    println!("{}", double(2));
    println!("{}", max(1, 2));
}
