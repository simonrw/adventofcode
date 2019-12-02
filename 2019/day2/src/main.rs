use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
enum Instruction {
    Add { lhs: i64, rhs: i64, dest: usize },
    Multiply { lhs: i64, rhs: i64, dest: usize },
    Terminal,
}

impl Instruction {
    fn from_elems(buf: &[i64]) -> Result<Instruction> {
        match buf[0] {
            1 => Ok(Instruction::Add {
                lhs: buf[1],
                rhs: buf[2],
                dest: buf[3] as _,
            }),
            2 => Ok(Instruction::Multiply {
                lhs: buf[1],
                rhs: buf[2],
                dest: buf[3] as _,
            }),
            _ => Err(format!("invalid instruction: {}", buf[0]).into()),
        }
    }
}

#[derive(Clone)]
struct RawInstructions(Vec<i64>);

impl RawInstructions {
    fn instructions(&self) -> Vec<Instruction> {
        let mut accum = Vec::with_capacity(4);
        let mut out = Vec::new();

        for elem in &self.0 {
            if accum.len() == 4 {
                eprintln!("{:?}", accum);
                out.push(Instruction::from_elems(&accum).unwrap());
                accum.clear();
                accum.push(*elem);
            } else if *elem == 99 {
                out.push(Instruction::Terminal);
            } else {
                accum.push(*elem);
            }
        }

        out

        // self.0.chunks(4).map(|elems| {
        //     if elems.len() != 4 {
        //         eprintln!("unexpected number of items: {:?}", elems);
        //         return None;
        //     }

        //     match &elems[0] {
        //         &1 => Some(Instruction::Add {
        //             lhs: elems[1],
        //             rhs: elems[2],
        //             dest: elems[3] as _,
        //         }),
        //         &2 => Some(Instruction::Multiply {
        //             lhs: elems[1],
        //             rhs: elems[2],
        //             dest: elems[3] as _,
        //         }),
        //         &99 => Some(Instruction::Terminal),
        //         _ => unreachable!(),
        //     }
        // })
    }

    fn set(&mut self, idx: usize, value: i64) {
        self.0[idx] = value;
    }

    fn result(&self) -> i64 {
        self.0[0]
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

    let mut memory = raw_instructions.clone();

    // Initial replacements
    memory.set(1, 12);
    memory.set(2, 2);

    let parsed = raw_instructions.instructions();

    for instruction in parsed {
        println!("{:?}", instruction);
    }

    println!("Final answer: {}", memory.result());
}

fn main() {
    part1();
}

#[cfg(test)]
mod tests {
    use super::*;
}
