#[macro_use]
extern crate nom;
mod grammar;

use nom::IResult;
use std::collections::HashMap;
use std::io::{self, Read};

use grammar::{Link, Value, OpType, instruction_kit};

type Wires<'a> = HashMap<&'a str, u16>;
type Dependencies<'a> = HashMap<&'a str, Vec<&'a Link<'a>>>;

trait ValueResolver {
    fn resolve(&self, v: &Value) -> Option<u16>;
}

impl<'a> ValueResolver for Wires<'a> {
    fn resolve(&self, v: &Value) -> Option<u16> {
        match *v {
            Value::Number(x) => Some(x),
            Value::WireRef(s) => {
                match self.get(s) {
                    Some(value) => Some(*value),
                    None => None,
                }
            }
        }
    }
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn get_signals<'a, 'b>(links: &'a Vec<Link<'b>>) -> Wires<'a> {
    let mut wires = Wires::new();

    for link in links.iter() {
        match link.input.op {
            OpType::LoadConstant => {
                if let Value::Number(constant) = link.input.arg1 {
                    wires.insert(link.output, constant);
                };
            }
            _ => (),
        }
    }

    wires
}

fn process_kit<'a, 'b>(wires: &'a mut Wires<'b>, links: &'a Vec<Link<'b>>) {
    let mut unresolved: bool = true;
    while unresolved {
        unresolved = false;

        for current in links {
            let arg1 = wires.resolve(&current.input.arg1);
            let arg2 = wires.resolve(&current.input.arg2);

            if arg1.is_none() || arg2.is_none() {
                unresolved = true;
                continue;
            };

            let val1 = arg1.unwrap();
            let val2 = arg2.unwrap();

            if let OpType::LoadConstant = current.input.op {
                if !wires.contains_key(&current.output) {
                    wires.insert(current.output, val1);
                }
            } else {
                let result: u16 = match current.input.op {
                    OpType::And => val1 & val2,
                    OpType::Or => val1 | val2,
                    OpType::Not => !val1,
                    OpType::LShift => val1 << val2,
                    OpType::RShift => val1 >> val2,
                    _ => val1,
                };

                wires.insert(current.output, result);
            }
        }
    }
}

fn day7(input: &String) {
    let cloned: String = input.clone();
    let bytestream = cloned.as_bytes();

    let links: Vec<Link> = match instruction_kit(bytestream) {
        IResult::Done(_, links) => links,
        _ => panic!("Parsing failed!"),
    };

    let mut wires = get_signals(&links);
    process_kit(&mut wires, &links);

    let wire_a = *wires.get(&"a").unwrap();
    println!("Wire 'a' after first iteration: {}", wire_a);

    wires = Wires::new();
    wires.insert("b", wire_a);
    process_kit(&mut wires, &links);

    println!("Wire 'a' after second iteration: {}",
             *wires.get(&"a").unwrap());
}


fn main() {
    let input = read_from_stdin();
    day7(&input);
}
