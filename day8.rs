use std::io::{self, Read};


fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn count_symbols(s: &str) -> (u8, u8) {
    let string = s.to_string();
    let mut seq = string.chars().peekable();

    let mut counter: Vec<(u8, u8)> = Vec::new();

    loop {
        let c = seq.next();

        if let None = c {
            return counter.iter()
                          .fold((0, 0),
                                |acc: (u8, u8), i: &(u8, u8)| (acc.0 + i.0, acc.1 + i.1));
        }

        let counts: (u8, u8) = match c {
            Some('\"') => (1, 0),
            Some('\\') => {
                let next_char = seq.peek().unwrap();
                match *next_char {
                    '\"' => (2, 1),
                    '\\' => (2, 1),
                    'x' => (4, 1),
                    _ => (0, 0),
                }
            }
            Some(_) => (1, 1),
            None => (0, 0),
        };
        for _ in 1..counts.0 {
            seq.next();
        }
        counter.push(counts);
    }
}

fn escape_string<'a>(s: &str) -> String {
    let result = s.replace("\\", "\\\\").replace("\"", "\\\"");
    result.to_owned()
}

fn main() {
    let input = read_from_stdin();
    let basic_tally = input.lines()
                           .map(count_symbols)
                           .fold((0, 0), |acc: (u32, u32), c: (u8, u8)| {
                               (acc.0 + (c.0 as u32), acc.1 + (c.1 as u32))
                           });
    println!("Basic tally: {:?}", basic_tally);

    let escaped_tally = input.lines()
                             .map(escape_string)
                             .map(|l| l.len() + 2)
                             .fold(0, |acc: usize, c: usize| acc + c);
    println!("Escaped tally: {:?}", escaped_tally);
}
