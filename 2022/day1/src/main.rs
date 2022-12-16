use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;

#[derive(Debug, Clone, Default)]
struct Elf {
    calorie_counts: Vec<u64>,
}

impl Elf {
    fn total_calories(&self) -> u64 {
        self.calorie_counts.iter().sum()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let reader = BufReader::new(Cursor::new(input));

    let mut elves = Vec::new();

    let mut current_elf = Elf::default();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            // the current elf is full
            elves.push(current_elf.clone());
            current_elf = Default::default();
        } else {
            let calorie_count = line.parse().unwrap();
            current_elf.calorie_counts.push(calorie_count);
        }
    }

    elves.sort_by_key(|elf| elf.total_calories());
    dbg!(&elves[elves.len() - 3..]
        .iter()
        .map(|elf| elf.total_calories())
        .sum::<u64>());
}
