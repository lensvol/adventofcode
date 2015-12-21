extern crate regex;

use std::collections::HashMap;

use std::io::{self, Read};
use regex::Regex;

static CMD_RE: &'static str = r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)";

type LightsMap = HashMap<(i32, i32), u8>;
type LightSwitch = fn(&mut LightsMap, i32, i32) -> ();

fn read_from_stdin() -> String {
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


fn brighten(state: &mut LightsMap, x: i32, y: i32) {
    let mut light = state.entry((x, y)).or_insert(0);
    *light += 1;
}


fn darken(state: &mut LightsMap, x: i32, y: i32) {
    let mut light = state.entry((x, y)).or_insert(0);
    if *light > 0 {
        *light -= 1;
    }
}

fn toggle(state: &mut LightsMap, x: i32, y: i32) {
    if !state.contains_key(&(x, y)) {
        state.insert((x, y), 1);
    } else {
        state.remove(&(x, y));
    }
}


fn brighten_by_2(state: &mut LightsMap, x: i32, y: i32) {
    let mut light = state.entry((x, y)).or_insert(0);
    *light += 2;
}


fn day6(input: &String, regulate_brightness: bool) -> LightsMap {
    let mut matrix: LightsMap = HashMap::new();
    let lines = input.lines();
    let cmd_re = Regex::new(CMD_RE).unwrap();

    let mut hard_switches: HashMap<&str, LightSwitch> = HashMap::new();
    hard_switches.insert("turn off", turn_off);
    hard_switches.insert("turn on", turn_on);
    hard_switches.insert("toggle", toggle);

    let mut soft_switches: HashMap<&str, LightSwitch> = HashMap::new();
    soft_switches.insert("turn on", brighten);
    soft_switches.insert("turn off", darken);
    soft_switches.insert("toggle", brighten_by_2);

    for line in lines {
        let cap = cmd_re.captures(line).unwrap();

        let cmd = cap.at(1).unwrap();
        let x1: i32 = cap.at(2).unwrap().to_string().parse().unwrap();
        let y1: i32 = cap.at(3).unwrap().to_string().parse().unwrap();

        let x2: i32 = cap.at(4).unwrap().to_string().parse().unwrap();
        let y2: i32 = cap.at(5).unwrap().to_string().parse().unwrap();

        let handler: Option<&LightSwitch> = match regulate_brightness {
            true => soft_switches.get(cmd),
            false => hard_switches.get(cmd),
        };

        if let Some(f) = handler {
            for y in y1..y2 + 1 {
                for x in x1..x2 + 1 {
                    f(&mut matrix, x, y);
                }
            }
        }
    }

    matrix
}

fn main() {
    let input = read_from_stdin();
    let binary_lights = day6(&input, false);
    let soft_lights = day6(&input, true);

    let total_brightness: i32 = soft_lights.values().fold(0, |acc, &br| acc + br as i32);

    println!("Lights on: {}", binary_lights.len());
    println!("Total brightness: {}", total_brightness);
}
