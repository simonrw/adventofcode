extern crate day20;

use day20::puzzle;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input_path = "input";
    let mut input = Vec::new();
    let mut file = File::open(input_path).expect(&format!("Could not open {}", input_path));
    file.read_to_end(&mut input).expect("Could not read from file");

    let n_addresses = 4294967295;
    let result = puzzle(&input, n_addresses).expect("Could not run puzzle code");
    println!("Minimum ip address: {}", result);
}
