use std::io::{self, Read};
use std::collections::{HashSet, HashMap};


type RouteMap<'a> = HashMap<(&'a str, &'a str), u16>;


trait Calculable<'a> {
    fn route_length(&self, path: &Vec<&'a str>) -> Option<u16>;
}


impl<'a> Calculable<'a> for RouteMap<'a> {
    fn route_length(&self, path: &Vec<&'a str>) -> Option<u16> {
        let mut total: u16 = 0;

        for i in 0..path.len() - 1 {
            let k = &(path[i], path[i + 1]);
            let entry = self.get(k);

            if let Some(distance) = entry {
                total = total + distance;
            } else {
                return None
            }
        }

        Some(total)
    }
}


fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}


fn permutate<'a>(n: usize, arr: &mut Vec<&'a str>, result: &mut Vec<Vec<&'a str>>) {
    if n == 1 {
        result.push(arr.clone());
    } else {
        for i in 0..n-1 {
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
            permutate(n - 1, arr, result);
        }
    }
}


fn heaps<'a>(strings: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut result: Vec<Vec<&'a str>> = vec![];
    let mut cloned = strings.clone();
    permutate(strings.len(), &mut cloned, &mut result);
    result.clone()
}


fn day9(input: &String) -> (u16, u16) {
    let mut routes: RouteMap = HashMap::new();
    let mut all_places: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let from_place: &str = parts[0];
        let to_place: &str = parts[2];
        let price: u16 = parts[4].parse::<u16>().unwrap();

        routes.insert((from_place, to_place), price);
        routes.insert((to_place, from_place), price);        

        all_places.insert(from_place);
        all_places.insert(to_place);
    };

    let mut list_of_places: Vec<&str> = vec![];
    for name in all_places.iter() {
        list_of_places.push(name);
    }
    list_of_places.sort();
    
    let permutated: Vec<Vec<&str>> = heaps(&list_of_places);
    let mut distances: Vec<u16> = permutated.iter()
                                            .map(|r| routes.route_length(r))
                                            .filter(|v| v.is_some())
                                            .map(|v| v.unwrap())
                                            .collect();

    distances.sort();
    (distances[0], distances[distances.len() - 1])
}


fn main() {
    let input = read_from_stdin();
    let (shortest, longest) = day9(&input);
    println!("Shortest: {}\nLongest: {}", shortest, longest);
}
