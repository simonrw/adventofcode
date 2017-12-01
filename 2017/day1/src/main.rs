use std::fs::File;
use std::io::Read;

fn main() {
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
