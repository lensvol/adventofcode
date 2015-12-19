use std::io::{self, Read};

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn day1(input: &String) -> (i32, Option<i32>) {
    let mut floor: i32 = 0;
    let mut position: i32 = 0;
    let mut basement_position: Option<i32> = None;
    
    for c in input.chars() {
        floor = match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        };

        position += 1;

        if floor == -1 {
            basement_position = match basement_position {
                Some(pos) => Some(pos),
                None => Some(position),
            }
        }
    }
    (floor, basement_position)
}

fn main() {
    let input = read_from_stdin();
    let (ends_up_at, enters_basement_at) =  day1(&input);
    println!("Ends up at {}", ends_up_at);
    if let Some(position) = enters_basement_at {
        println!("Enters basement at {}", position);
    };
}
