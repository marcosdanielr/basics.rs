use std::io;

fn convert_to_int(input: &String) -> i32 {
    let result = input.trim().parse::<i32>().unwrap();

    result
}

fn main() {
    let mut mediums: String = String::new();
    io::stdin()
        .read_line(&mut mediums)
        .expect("Error on read mediums");

    let mut recovery_students: i128 = 0;
    let mut i = 0;

    while convert_to_int(&mediums) > i {
        let mut student_medium = String::new();
        io::stdin()
            .read_line(&mut student_medium)
            .expect("Error on read student_medium");

        i += 1;

        if convert_to_int(&student_medium) >= 3 && convert_to_int(&student_medium) < 6 {
            recovery_students += 1;
        }
    }

    println!("Has {} recovery students", recovery_students);
}
