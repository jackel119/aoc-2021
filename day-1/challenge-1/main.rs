use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut count = 0;
    let mut last: Option<u32> = None;
    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        let parsed: u32 = line.parse().unwrap();

        match last {
            Some(x) => {
                if parsed > x {
                    count += 1;
                }
            },
            None => {} // Do nothing,
        };

        last = Some(parsed);
    }

    println!("{}", count);
}
