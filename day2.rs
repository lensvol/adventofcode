use std::cmp::min;
use std::io::{self, Read};

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    buffer
}

struct GiftBox {
    length: i32,
    width: i32,
    height: i32,
}

impl GiftBox {
    pub fn surface_area(&self) -> i32 {
        (2 * self.length * self.width) + (2 * self.width * self.height) +
        (2 * self.height * self.length)
    }

    pub fn smallest_side(&self) -> i32 {
        let side1 = self.length * self.width;
        let side2 = self.width * self.height;
        let side3 = self.height * self.length;
        min(min(side1, side2), side3)
    }

    pub fn smallest_perimeter(&self) -> i32 {
        let side1 = (self.length + self.width) * 2;
        let side2 = (self.width + self.height) * 2;
        let side3 = (self.height + self.length) * 2;

        min(min(side1, side2), side3)
    }

    pub fn smallest_distance(&self) -> i32 {
        let distance1 = (self.length + self.width) * 2;
        let distance2 = (self.height + self.width) * 2;
        let distance3 = (self.length + self.height) * 2;
        min(min(distance1, distance2), distance3)
    }

    pub fn volume(&self) -> i32 {
        self.length * self.width * self.height
    }

    pub fn new(dimensions: &str) -> GiftBox {
        let parts: Vec<i32> = dimensions.split("x").map(|s| s.parse::<i32>().unwrap()).collect();
        GiftBox {
            length: parts[0],
            width: parts[1],
            height: parts[2],
        }
    }
}

fn day2(input: &String) -> (i32, i32) {
    let lines = input.lines();
    let mut total_paper: i32 = 0;
    let mut total_ribbon: i32 = 0;
    let boxes: Vec<GiftBox> = lines.map(|dim| GiftBox::new(dim)).collect();

    for b in boxes {
        total_paper += b.surface_area() + b.smallest_side();
        total_ribbon += min(b.smallest_perimeter(), b.smallest_distance()) + b.volume();
    }
    (total_paper, total_ribbon)
}

fn main() {
    let input = read_from_stdin();
    let (paper, ribbon): (i32, i32) = day2(&input);
    println!("Elves need {} square feet of wrapping paper and {} feet of ribbon ",
             paper,
             ribbon);
}
