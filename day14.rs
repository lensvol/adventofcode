use std::io::{self, Read};
use std::collections::HashMap;

type HappinessMap<'a> = HashMap<(&'a str, &'a str), i32>;

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}


struct Reindeer<'a> {
    name: &'a str,
    speed: u32,
    run_time: u32,
    rest_time: u32,
}


fn load_reindeer_info<'a>(data: &'a str) -> Vec<Reindeer> {
    let lines = data.lines();
    let mut reindeers: Vec<Reindeer> = vec![];
    
    for l in lines {
        let parts = l.split(" ").collect::<Vec<&str>>();

        let name: &str = parts[0];
        let speed: u32 = parts[3].parse::<u32>().unwrap();;
        let run_time: u32 = parts[6].parse::<u32>().unwrap();
        let rest_time: u32 = parts[13].parse::<u32>().unwrap();

        reindeers.push(Reindeer{
            name: name,
            speed: speed,
            run_time: run_time,
            rest_time: rest_time,
        });
    }

    reindeers
}


fn day15<'a>(pack: &Vec<Reindeer>, race_length: u32) -> u32 {
    let mut maximum_distance: u32 = 0;

    for reindeer in pack {
        let full_cycles = race_length / (reindeer.run_time + reindeer.rest_time);
        let remainder = race_length % (reindeer.run_time + reindeer.rest_time);

        let mut distance = full_cycles * reindeer.run_time * reindeer.speed;
        if remainder <= reindeer.run_time {
            distance += remainder * reindeer.speed;
        } else {
            distance += reindeer.run_time * reindeer.speed;
        }

        println!("{} -> {} km", reindeer.name, distance);

        if maximum_distance < distance {
            maximum_distance = distance;
        }
    }

    maximum_distance
}

enum DeerState {
    Running,
    Resting,
}

struct State<'a> {
    deer: &'a Reindeer<'a>,
    current: DeerState,
    points: u32,
    duration: u32,
    distance: u32,
}

impl<'a> State<'a> {
    fn tick(&mut self) {
        self.duration += 1;

        if let DeerState::Running = self.current {
            self.distance += self.deer.speed;
        }

        match self.current {
            DeerState::Running => {
                if self.duration == self.deer.run_time {
                    self.current = DeerState::Resting;
                    self.duration = 0;
                }
            },
            DeerState::Resting => {
                if self.duration == self.deer.rest_time {
                    self.current = DeerState::Running;
                    self.duration = 0;
                }
            },
        }
    }
}

fn day15_points(pack: &Vec<Reindeer>, race_length: u32) -> u32 {
    let mut states: Vec<State> = vec![];
    let mut maximum_distance: u32 = 0;
    
    for r in pack {
        states.push(State{
            deer: &r,
            points: 0,
            duration: 0,
            current: DeerState::Running,
            distance: 0,
        })
    };
    
    for _ in 0..race_length {
        for state in states.iter_mut() {
            state.tick();
            if state.distance > maximum_distance {
                maximum_distance = state.distance;
            }
        }

        for state in states.iter_mut() {
            if state.distance == maximum_distance {
                state.points += 1;
            }
        }
    }

    for state in states {
        println!("{} -> {} ({} points)", state.deer.name, state.distance, state.points);
    }
    
    42
}

fn main() {
    let input = read_from_stdin();
    let reindeers = load_reindeer_info(&input);
    println!("Winner travelled {} km.", day15(&reindeers, 2503));
    println!("Points: {}", day15_points(&reindeers, 2503)); 
}
