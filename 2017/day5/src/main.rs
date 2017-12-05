#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::Read;

type Instruction = i32;


fn main() {
    let mut s = String::new();
    let mut f = File::open("input.txt").expect("opening input file");
    f.read_to_string(&mut s).expect("reading file contents");

    timeit(|| {
        println!("PART 1: {}", final_instruction_part_1(&s));
    });

    timeit(|| {
        println!("PART 2: {}", final_instruction_part_2(&s));
    });
}

fn parse_instructions(text: &str) -> Vec<Instruction> {
    text.split_whitespace()
        .map(|c| c.parse().expect("parsing integer"))
        .collect()
}

fn timeit<F>(f: F)
where
    F: Fn(),
{
    use std::time::Instant;
    let now = Instant::now();
    f();
    let elapsed = now.elapsed();
    let secs = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
    eprintln!("* msec: {}", secs * 1000.0);
}

fn final_instruction_part_1(text: &str) -> u32 {
    let mut instructions = parse_instructions(text);
    let mut pos: i32 = 0;
    let mut steps = 0;

    while let Some(i) = instructions.get_mut(pos as usize) {
        pos += *i;
        *i += 1;
        steps += 1;
    }

    steps
}

fn final_instruction_part_2(text: &str) -> u32 {
    let mut instructions = parse_instructions(text);
    let mut pos: i32 = 0;
    let mut steps = 0;

    while let Some(i) = instructions.get_mut(pos as usize) {
        pos += *i;
        if *i >= 3 {
            *i -= 1;
        } else {
            *i += 1;
        }
        steps += 1;
    }

    steps
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_parse_instructions() {
        let text = "0\n3\n0\n1\n3\n";
        assert_eq!(parse_instructions(text), vec![0, 3, 0, 1, 3]);
    }

    #[test]
    fn test_example_result() {
        let text = "0\n3\n0\n1\n3\n";
        assert_eq!(final_instruction_part_1(text), 4);
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        let mut s = String::new();
        let mut f = File::open("input.txt").expect("opening input file");
        f.read_to_string(&mut s).expect("reading file contents");

        b.iter(|| final_instruction_part_1(&s));
    }

    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        let mut s = String::new();
        let mut f = File::open("input.txt").expect("opening input file");
        f.read_to_string(&mut s).expect("reading file contents");

        b.iter(|| final_instruction_part_2(&s));
    }
}
