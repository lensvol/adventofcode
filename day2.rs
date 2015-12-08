use std::cmp::min;
use std::io::{self, Read};

fn read_from_stdin() -> String{
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
        (2 * self.length * self.width) + (2 * self.width * self.height) + (2 * self.height * self.length)
    }

    pub fn smallest_side(&self) -> i32 {
        let side1 = self.length * self.width;
        let side2 = self.width * self.height;
        let side3 = self.height * self.length;
        min(
            min(side1, side2),
            side3,
        )
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

fn day2(input: String) -> i32 {
    let lines = input.lines();

    let boxes: Vec<GiftBox> = lines.map(|dim| GiftBox::new(dim)).collect();
    let paper_needed: Vec<i32> = boxes.iter().map(|b| b.surface_area() + b.smallest_side()).collect();
    paper_needed.iter().fold(0, |a, &b| a + b)
}

fn main() {
    let input = read_from_stdin();
    println!("{}", day2(input));
}
