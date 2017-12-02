use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &'static str = "input.txt";

struct Stats {
    min: u32,
    max: u32,
    range: Option<u32>,
}

fn parse_ints(ints: Vec<u32>) -> Stats {
    let mut min = std::u32::MAX;
    let mut max = std::u32::MIN;

    for int in ints.iter() {
        if *int < min {
            min = *int;
        }

        if *int > max {
            max = *int;
        }
    }

    Stats {
        min: min,
        max: max,
        range: Some(max - min),
    }
}

fn main() {
    let f = File::open(FILENAME).unwrap();
    let reader = BufReader::new(f);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split_whitespace().collect();
        let ints: Vec<u32> = words.iter().map(|word| word.parse().unwrap()).collect();
        let stats = parse_ints(ints);
        match stats.range {
            Some(value) => total += value,
            None => panic!("should have value"),
        }
    }

    println!("TOTAL: {}", total);
}
