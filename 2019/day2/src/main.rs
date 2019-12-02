use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
struct RawInstructions(Vec<i64>);

#[derive(Debug)]
struct MemoryLocation(usize);

impl RawInstructions {
    fn set(&mut self, idx: MemoryLocation, value: i64) {
        log::debug!("setting memory {:?} to {}", idx, value);
        self.0[idx.0] = value;
    }

    fn result(&self) -> i64 {
        self.0[0]
    }

    fn chunks<'a>(&'a self, n: usize) -> std::slice::Chunks<'a, i64> {
        self.0.chunks(n)
    }

    fn get(&self, idx: MemoryLocation) -> i64 {
        self.0[idx.0]
    }
}

fn read_input_data<S>(fname: S) -> Result<RawInstructions>
where
    S: AsRef<Path>,
{
    let input_data_text = fs::read_to_string(fname)?;
    let input_data: Vec<_> = input_data_text
        .trim()
        .split(',')
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect();
    Ok(RawInstructions(input_data))
}

fn part1() {
    let mut raw_instructions = read_input_data("input.txt").unwrap();
    raw_instructions.set(MemoryLocation(1), 12);
    raw_instructions.set(MemoryLocation(2), 2);
    println!("result: {}", analyse_instructions(&raw_instructions));
}

fn analyse_instructions(instructions: &RawInstructions) -> i64 {
    let mut memory = instructions.clone();

    for chunk in instructions.chunks(4) {
        log::debug!("--- chunk: {:?}", chunk);
        match chunk[0] {
            1 => {
                log::info!("got add instruction");
                let lhs = memory.get(MemoryLocation(chunk[1] as _));
                let rhs = memory.get(MemoryLocation(chunk[2] as _));
                let dest = chunk[3];
                memory.set(MemoryLocation(dest as _), lhs + rhs);
            }
            2 => {
                log::info!("got multiply instruction");
                let lhs = memory.get(MemoryLocation(chunk[1] as _));
                let rhs = memory.get(MemoryLocation(chunk[2] as _));
                let dest = chunk[3];
                memory.set(MemoryLocation(dest as _), lhs * rhs);
            }
            99 => {
                log::info!("reached terminal");
                break;
            }
            e => {
                log::error!("invalid chunk starting element: {}", e);
            }
        }
    }

    memory.result()
}

fn find_target_inputs(ins: &RawInstructions, target: i64) -> (i64, i64) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut new_ins = ins.clone();
            new_ins.set(MemoryLocation(1), noun);
            new_ins.set(MemoryLocation(2), verb);

            if analyse_instructions(&new_ins) == target {
                return (noun, verb);
            }
        }
    }

    unreachable!();
}

fn part2() {
    let original_instructions = read_input_data("input.txt").unwrap();
    let (noun, verb) = find_target_inputs(&original_instructions, 19690720);
    println!("Result: {}", 100 * noun + verb);
}

fn main() {
    env_logger::init();
    // part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let instructions = RawInstructions(vec![1, 0, 0, 0, 99]);
        assert_eq!(analyse_instructions(&instructions), 2);
    }
}
