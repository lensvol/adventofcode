use std::io::{self, Read};

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn day1(input: String) -> i32 {
    let mut floor: i32 = 0;
    for c in input.chars() {
        floor = match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        };
    }
    floor
}

fn main() {
    let input = read_from_stdin();
    println!("{}", day1(input));
}
