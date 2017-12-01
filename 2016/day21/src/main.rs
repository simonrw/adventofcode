extern crate day21;

use day21::puzzle;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input_path = "input";
    let mut input = Vec::new();
    let mut file = File::open(input_path).expect(&format!("Could not open {}", input_path));
    file.read_to_end(&mut input).expect("Could not read from file");

    let result = puzzle(&input).expect("Could not run puzzle code");
}
