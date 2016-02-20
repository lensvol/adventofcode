use std::io::{self, Read};

#[derive(Debug)]
struct Rotator {
    letters: Vec<u8>,
}

impl Rotator {
    pub fn new(init_state: &str) -> Rotator {
        Rotator { letters: init_state.as_bytes().to_vec() }
    }
}

impl Iterator for Rotator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut pointer = self.letters.len() - 1;
        while pointer > 0 {
            let c = self.letters[pointer] as char;

            match c {
                'z' => {
                    self.letters[pointer] = 'a' as u8;
                    pointer -= 1;
                }
                _ => {
                    self.letters[pointer] += 1;
                    break;
                }
            }
        }

        let result: String = String::from_utf8(self.letters.clone()).unwrap();
        Some(result)
    }
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn is_good_enough(variant: &String) -> bool {
    let mut previous: char = ' ';
    let mut previous_pair_position: u8 = 0;
    let mut previous_pair_character: char = ' ';

    let mut pairs_found: u8 = 0;
    let mut consecutive_found: bool = false;
    let mut consecutive_pairs: u8 = 0;

    let mut position: u8 = 0;

    for c in variant.chars() {
        position += 1;

        if previous == c {
            if previous_pair_position > 0 {
                if position - previous_pair_position < 2 || previous_pair_character == c {
                    return false;
                }
            } else {
                previous_pair_position = position;
                previous_pair_character = c;
            }
            pairs_found += 1;
            consecutive_pairs = 0;
        } else {
            let c_code = c as u8;
            let prev_code = previous as u8;

            if c_code < prev_code || c_code - prev_code > 1 {
                consecutive_pairs = 0;
            } else {
                consecutive_pairs += 1;
            }

            if consecutive_pairs >= 2 {
                consecutive_found = true;
            }
        }

        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        previous = c;
    }

    pairs_found >= 2 && consecutive_found
}

fn main() {
    let r = Rotator::new(&read_from_stdin().trim_right());
    let good_passwords: Vec<String> = r.filter(is_good_enough).take(2).collect();
    println!("Next two valid passwords are: {:?}", good_passwords);
}
