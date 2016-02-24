use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

type HappinessMap<'a> = HashMap<(&'a str, &'a str), i32>;

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

fn permutate<'a>(n: usize, arr: &mut Vec<&'a str>, result: &mut Vec<Vec<&'a str>>) {
    if n == 1 {
        result.push(arr.clone());
    } else {
        for i in 0..n - 1 {
            permutate(n - 1, arr, result);
            if n % 2 == 0 {
                let temp = arr[i];
                arr[i] = arr[n - 1];
                arr[n - 1] = temp;
            } else {
                let temp = arr[0];
                arr[0] = arr[n - 1];
                arr[n - 1] = temp;
            }
        }
        permutate(n - 1, arr, result);
    }
}


fn heaps<'a>(strings: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut result: Vec<Vec<&'a str>> = vec![];
    let mut cloned = strings.clone();
    permutate(strings.len(), &mut cloned, &mut result);
    result.clone()
}


fn load_relationships<'a>(data: &'a str) -> (Vec<&str>, HappinessMap<'a>) {
    let lines = data.lines();
    let mut relationships = HappinessMap::new();
    let mut buffer: HashSet<&str> = HashSet::new();

    for l in lines {
        let parts = l.split(" ").collect::<Vec<&str>>();
        let who = parts[0];
        let another: &str = parts.last().unwrap();

        let action = parts[2];
        let mut points = parts[3].parse::<i32>().unwrap();

        if action == "lose" {
            points = -points;
        }

        buffer.insert(who);
        buffer.insert(&another[0..another.len() - 1]);

        relationships.insert((who, &another[0..another.len() - 1]), points);
    }

    let guests = buffer.drain().collect::<Vec<&str>>();

    (guests, relationships)
}


fn calculate_happiness(table: &Vec<&str>, relationships: &HappinessMap) -> i32 {
    let mut total: i32 = 0;

    for i in 0..table.len() - 1 {
        let entry = relationships.get(&(table[i], table[i + 1]));
        if let Some(points) = entry {
            total = total + points;
        }

        let entry2 = relationships.get(&(table[i + 1], table[i]));
        if let Some(points) = entry2 {
            total = total + points;
        }
    }

    let last_first = relationships.get(&(table[0], table[table.len() - 1])).unwrap();
    let first_last = relationships.get(&(table[table.len() - 1], table[0])).unwrap();

    total + last_first + first_last
}


fn main() {
    let input = read_from_stdin();
    let (mut guests, mut relationships) = load_relationships(&input[..]);

    let variants = heaps(&guests);
    let mut points = variants.iter()
                             .map(|v| calculate_happiness(&v, &relationships))
                             .collect::<Vec<i32>>();
    points.sort();
    println!("Maximum possible happiness: {}", points.last().unwrap());

    for guest in guests.iter() {
        relationships.insert((&guest, "Me"), 0);
        relationships.insert(("Me", &guest), 0);
    }
    guests.insert(0, "Me");

    let variants = heaps(&guests);
    points = variants.iter().map(|v| calculate_happiness(&v, &relationships)).collect::<Vec<i32>>();
    points.sort();
    println!("Maximum possible happiness with me: {}",
             points.last().unwrap());
}
