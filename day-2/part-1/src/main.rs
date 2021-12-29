use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let mut y = 0;
    let mut x = 0;

    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        let parsed: Vec<&str> = line.split(" ").collect();

        let action = parsed[0];
        let value: i32  = parsed[1].parse().unwrap();

        match action {
            "up" => x -= value,
            "down" => x += value,
            "forward" => y += value,
            _ => {},
        }
    }

    println!("x = {}, y = {}", x, y);
    println!("x * y = {}", x * y);
}
