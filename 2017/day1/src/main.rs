use std::fs::File;
use std::io::Read;

/*
fn part_one() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut first_char: u32 = 0;
    let mut found_first = false;

    let mut previous_char: u32 = 0;
    let mut is_first_char = true;
    let mut total = 0;

    for c in s.chars() {
        let intval = c.to_digit(10).unwrap();
        if !found_first {
            found_first = true;
            first_char = intval;
        }

        if is_first_char {
            is_first_char = false;
            previous_char = intval;
            continue;
        }

        if previous_char == intval {
            total += intval;
        }
        previous_char = intval;
    }
    println!("{}", total);
}
*/

fn part_two() {
    let mut f = File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let digits: Vec<_> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n_digits = digits.len();
    let half_value = n_digits / 2;

    let mut total = 0;
    for i in 0..n_digits {
        let value = digits[i];

        let other_idx = (i + half_value) % n_digits;
        let other_value = digits[other_idx];
        if value == other_value {
            total += value;
        }
    }

    println!("{}", total);
}

fn main() {
    // part_one();
    part_two();
}
