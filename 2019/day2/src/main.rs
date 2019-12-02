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
    let raw_instructions = read_input_data("input.txt").unwrap();
    println!("result: {}", analyse_instructions(&raw_instructions));
}

fn analyse_instructions(instructions: &RawInstructions) -> i64 {
    let mut memory = instructions.clone();

    // Initial replacements
    memory.set(MemoryLocation(1), 12);
    memory.set(MemoryLocation(2), 2);

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

fn main() {
    env_logger::init();
    part1();
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
