use std::io::{self, Read};

fn read_from_stdin() -> String{
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

#[derive(PartialEq, Debug)]
enum Verdict {
    Naughty,
    Nice,
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn naughty_or_nice(input: String) -> Verdict{
    let mut previous: char = 0 as char;
    let mut vowel_count: i32 = 0;
    let mut has_doubles: bool = false;
    
    for c in input.chars() {
        if c == previous {
            has_doubles = true;
        }

        if is_vowel(c) {
            vowel_count += 1;
        }

        if (c == 'b' && previous == 'a') ||
            (c == 'd' && previous == 'c') ||
            (c == 'q' && previous == 'p') ||
            (c == 'y' && previous == 'x') {
            return Verdict::Naughty;
        }
        previous = c;
    }

    if has_doubles && vowel_count >= 3 {
        Verdict::Nice
    } else {
        Verdict::Naughty
    }
}

fn day5(input: String) -> i32 {
    let lines = input.lines();
    
    let nice_verdicts: Vec<Verdict> = lines.map(|candidate| naughty_or_nice(candidate.to_string())).filter(|v| *v == Verdict::Nice).collect();
    nice_verdicts.len() as i32
}

fn main() {
    let input = read_from_stdin();
    println!("Nice strings: {}", day5(input));
}
