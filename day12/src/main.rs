extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::io::{self, Read};

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn sum_traverse(data: &Json, ignore_red: bool) -> i64 {
    let mut total: i64 = 0;

    if let Some(array) = data.as_array() {
        let parts: Vec<i64> = array.iter()
                                   .map(|x| sum_traverse(x, ignore_red))
                                   .collect::<Vec<i64>>();
        total = parts.iter().fold(0 as i64, |sum, it| sum + it);
    } else if let Some(obj) = data.as_object() {
        if ignore_red {
            for (_, value) in obj.iter() {
                if let Some(s) = value.as_string() {
                    if s == "red" {
                        return 0;
                    }
                }

            }
        };

        for (_, value) in obj.iter() {
            total += sum_traverse(value, ignore_red);
        }
    } else if let Some(v) = data.as_i64() {
        total = v;
    }

    total
}

fn main() {
    let input = read_from_stdin();

    match Json::from_str(&input[..]) {
        Ok(data) => println!("Including everything: {:?}", sum_traverse(&data, false)),
        Err(e) => println!("Can't parse JSON: {}", e),
    }

    match Json::from_str(&input[..]) {
        Ok(data) => println!("Without red: {:?}", sum_traverse(&data, true)),
        Err(e) => println!("Can't parse JSON: {}", e),
    }
}
