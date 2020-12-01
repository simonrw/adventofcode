use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct MemoryLocation(i64);

fn read_input_data<S>(fname: S) -> Result<Vec<i64>>
where
    S: AsRef<Path>,
{
    let input_data_text = fs::read_to_string(fname)?;
    let input_data: Vec<_> = input_data_text
        .trim()
        .split(',')
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect();
    Ok(input_data)
}

#[derive(Debug)]
enum Opcode {
    Sentinel,
    Add(MemoryLocation, i64, i64),
    Mul(MemoryLocation, i64, i64),
}

struct IntcodeComputer {
    memory: Vec<i64>,
    i: Option<i64>,
    idx: usize,
    buf: Vec<i64>,
}

impl IntcodeComputer {
    fn new(arr: &[i64]) -> Self {
        Self {
            memory: Vec::from(arr),
            i: None,
            idx: 0,
            buf: Vec::with_capacity(4),
        }
    }

    fn set(&mut self, dest: MemoryLocation, value: i64) {
        log::debug!("setting pos {:?} to {}", dest, value);
        self.memory[dest.0 as usize] = value;
    }

    fn add(&mut self, dest: MemoryLocation, lhs: i64, rhs: i64) {
        let lhs = self.memory[lhs as usize];
        let rhs = self.memory[rhs as usize];
        self.set(dest, lhs + rhs);
    }

    fn mul(&mut self, dest: MemoryLocation, lhs: i64, rhs: i64) {
        let lhs = self.memory[lhs as usize];
        let rhs = self.memory[rhs as usize];
        self.set(dest, lhs * rhs);
    }

    fn result(&self) -> Option<&i64> {
        self.memory.get(0)
    }

    fn next(&mut self) -> Option<Opcode> {
        loop {
            self.i = Some(self.memory[self.idx]);

            if self.idx >= self.memory.len() {
                return None;
            }

            if let Some(99) = self.i {
                self.idx += 1;
                return Some(Opcode::Sentinel);
            }

            if self.buf.len() == 4 {
                log::debug!("complete opcode: {:?}", self.buf);
                // We should have a complete instruction opcode
                if self.buf[0] == 1 {
                    let res = Opcode::Add(MemoryLocation(self.buf[3]), self.buf[1], self.buf[2]);
                    self.buf.clear();
                    return Some(res);
                } else if self.buf[0] == 2 {
                    let res = Opcode::Mul(MemoryLocation(self.buf[3]), self.buf[1], self.buf[2]);
                    self.buf.clear();
                    return Some(res);
                } else {
                    unreachable!("{:?}", self.buf)
                }
            }

            self.idx += 1;

            if let Some(value) = self.i {
                self.buf.push(value);
            }
        }
    }
}

fn main() {
    env_logger::init();
    let raw_instructions = read_input_data("input.txt").unwrap();
    let mut computer = IntcodeComputer::new(raw_instructions.as_ref());

    computer.set(MemoryLocation(1), 12);
    computer.set(MemoryLocation(2), 2);

    loop {
        match computer.next() {
            None => break,
            Some(opcode) => {
                log::debug!("got opcode {:?}", opcode);
                match opcode {
                    Opcode::Sentinel => break,
                    Opcode::Add(MemoryLocation(dest), lhs, rhs) => {
                        computer.add(MemoryLocation(dest), lhs, rhs)
                    }
                    Opcode::Mul(MemoryLocation(dest), lhs, rhs) => {
                        computer.mul(MemoryLocation(dest), lhs, rhs)
                    }
                }
            }
        }
    }

    println!("computer result: {}", computer.result().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let raw_instructions = read_input_data("input.txt").unwrap();
        let mut computer = IntcodeComputer::new(raw_instructions.as_ref());

        computer.set(MemoryLocation(1), 12);
        computer.set(MemoryLocation(2), 2);

        loop {
            match computer.next() {
                None => break,
                Some(opcode) => {
                    log::debug!("got opcode {:?}", opcode);
                    match opcode {
                        Opcode::Sentinel => break,
                        Opcode::Add(dest, lhs, rhs) => computer.add(dest, lhs, rhs),
                        Opcode::Mul(dest, lhs, rhs) => computer.mul(dest, lhs, rhs),
                    }
                }
            }
        }

        assert_eq!(*computer.result().unwrap(), 4138658);
    }
}
