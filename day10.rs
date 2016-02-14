use std::io::{self, Read};
use std::char::from_digit;

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn count_digits(input: &String) -> String {
    let mut counter: u32 = 0;
    let mut result: Vec<char> = vec![];
    let mut current_char: char = ' ';
    
    for c in input.trim().chars() {
        if current_char == ' ' {
            current_char = c;
        };
        
        if current_char == c {
            counter += 1;
        } else {
            result.push(from_digit(counter, 10).unwrap());
            result.push(current_char);

            current_char = c;
            counter = 1;
        };
    }

    result.push(from_digit(counter, 10).unwrap());
    result.push(current_char);

    result.into_iter().collect()
}

fn day10(input: &String, times: u8) -> String {
    let mut result = count_digits(input);

    for _ in 1..times {
        result = count_digits(&result);
    }
    result
}

fn main() {
    let input = read_from_stdin();
    println!("{}", day10(&input, 40).len());
    println!("{}", day10(&input, 50).len());
}
