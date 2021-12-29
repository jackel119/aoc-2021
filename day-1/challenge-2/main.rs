use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut count = 0;
    
    let mut last: Option<u32> = None;
    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut depths: Vec<u32> = vec![];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        let parsed: u32 = line.parse().unwrap();

        depths.push(parsed);
    }

    for i in 0..depths.len() {
        if i <= 1 { continue; };

        let sum = depths[i] + depths[i - 1] + depths[i - 2];
        match last {
            Some(x) => {
                if sum > x { count += 1; }
            },
            None => {},
        }

        last = Some(sum)
    }

    println!("{}", count);
}
