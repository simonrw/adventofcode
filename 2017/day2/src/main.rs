use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &'static str = "input.txt";

struct Stats1 {
    min: u32,
    max: u32,
    range: Option<u32>,
}

fn parse_ints_part1(ints: &[u32]) -> Stats1 {
    let mut min = std::u32::MAX;
    let mut max = std::u32::MIN;

    for int in ints {
        if *int < min {
            min = *int;
        }

        if *int > max {
            max = *int;
        }
    }

    Stats1 {
        min: min,
        max: max,
        range: Some(max - min),
    }
}

fn parse_ints_part2(ints: &[u32]) -> u32 {
    for start_idx in 0..ints.len() {
        let start = ints[start_idx];
        for end_idx in 0..ints.len() {
            if start_idx == end_idx {
                continue;
            }

            let end = ints[end_idx];
            let remainder = end % start;

            if remainder == 0 {
                return end / start;
            }
        }
    }

    panic!("SHOULD NOT GET HERE");
}

fn main() {
    let f = File::open(FILENAME).unwrap();
    let reader = BufReader::new(f);

    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split_whitespace().collect();
        let ints: Vec<u32> = words.iter().map(|word| word.parse().unwrap()).collect();

        let stats = parse_ints_part1(&ints);
        match stats.range {
            Some(value) => part1_total += value,
            None => panic!("should have value"),
        }

        part2_total += parse_ints_part2(&ints);
    }

    println!("PART 1 TOTAL: {}", part1_total);
    println!("PART 2 TOTAL: {}", part2_total);
}
