extern crate regex;

use std::collections::HashMap;

use std::io::{self, Read};
use regex::Regex;

static CMD_RE: &'static str = r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)";

type LightsMap = HashMap<(i32, i32), u8>;


fn read_from_stdin() -> String{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}


fn turn_on(state: &mut LightsMap, x: i32, y: i32) {
    if !state.contains_key(&(x, y)) {
        state.insert((x, y), 1);
    }
}


fn turn_off(state: &mut LightsMap, x: i32, y: i32) {
    state.remove(&(x, y));
}


fn toggle(state: &mut LightsMap, x: i32, y: i32) {
    if !state.contains_key(&(x, y)) {
        state.insert((x, y), 1);
    } else {
        state.remove(&(x, y));
    }
}


fn day6(input: String) -> usize {
    let mut matrix: LightsMap = HashMap::new();
    let lines = input.lines();
    let cmd_re = Regex::new(CMD_RE).unwrap();

    for line in lines {
        println!("{}", line);
        let cap = cmd_re.captures(line).unwrap();

        let cmd = cap.at(1).unwrap();
        let x1: i32 = cap.at(2).unwrap().to_string().parse().unwrap();
        let y1: i32 = cap.at(3).unwrap().to_string().parse().unwrap();

        let x2: i32 = cap.at(4).unwrap().to_string().parse().unwrap();
        let y2: i32 = cap.at(5).unwrap().to_string().parse().unwrap();        

        let handler: Option<fn(&mut LightsMap, i32, i32) -> ()> = match cmd {
            "turn off" => Some(turn_off),
            "turn on" => Some(turn_on),
            "toggle" => Some(toggle),
            _ => None,
        };

        if let Some(f) = handler {
            for y in y1..y2 + 1{
                for x in x1..x2 + 1{
                    f(&mut matrix, x, y);
                }
            }
        }
    }
    
    matrix.len()
}

fn main() {
    let input = read_from_stdin();
    println!("Lights on: {}", day6(input));
}
