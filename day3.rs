use std::io::{self, Read};
use std::collections::HashMap;

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn day3(input: &String) -> usize {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut visited_houses = HashMap::new();
    visited_houses.insert("0,0".to_owned(), true);

    for c in input.chars() {
        let (dx, dy) = match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => (0, 0),
        };

        x += dx;
        y += dy;

        let key = format!("{},{}", x, y);

        if !visited_houses.contains_key(&key) {
            visited_houses.insert(key.to_owned(), true);
        }
    }
    visited_houses.len()
}

fn day3_part2(input: &String) -> usize {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut robot_x: i32 = 0;
    let mut robot_y: i32 = 0;

    let mut is_robot: bool = false;

    let mut visited_houses = HashMap::new();
    visited_houses.insert("0,0".to_owned(), true);

    for c in input.chars() {
        let (dx, dy) = match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => (0, 0),
        };

        let key: String;

        if is_robot {
            robot_x += dx;
            robot_y += dy;
            key = format!("{},{}", robot_x, robot_y);
        } else {
            x += dx;
            y += dy;
            key = format!("{},{}", x, y);
        }
        is_robot = !is_robot;

        if !visited_houses.contains_key(&key) {
            visited_houses.insert(key.to_owned(), true);
        }
    }
    visited_houses.len()
}

fn main() {
    let input = read_from_stdin();
    println!("{}", day3(&input));
    println!("{}", day3_part2(&input));
}
