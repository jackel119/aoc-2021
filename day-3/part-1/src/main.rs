use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut bit_counts: Vec<usize> = vec![];

    let filename = "input.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let total = lines.len();

    for (index, line) in lines.into_iter().enumerate() {
        if index == 0 {
            bit_counts = line.chars().map(|_| 0).collect();
        }

        for (i, c) in line.chars().enumerate() {
            bit_counts[i] = bit_counts[i] + c.to_digit(10).unwrap() as usize;
        }
    }


    let gamma_bits: Vec<usize> = bit_counts
        .clone()
        .into_iter()
        .map(|x| if x > total / 2 { 1 } else { 0 })
        .collect();

    let gamma = to_u32(&gamma_bits);

    let epsilon_bits: Vec<usize> = bit_counts
        .into_iter()
        .map(|x| if x > total / 2 { 0 } else { 1 })
        .collect();
    let epsilon = to_u32(&epsilon_bits);

    println!("{:?}", gamma_bits);
    println!("{}", gamma);

    println!("{:?}", epsilon_bits);
    println!("{}", epsilon);
    println!("{}", epsilon * gamma);
}

fn to_u32(slice: &[usize]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}
