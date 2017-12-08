extern crate combine;

use combine::{Parser, State};
use combine::char::{digit, letter};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Debug, Eq, PartialEq)]
pub enum InstructionType {
    Increment,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Condition {
    register: String,
    operation: Operation,
    value: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    GreaterThan,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    register: String,
    instruction: InstructionType,
    condition: Condition,
}

impl Instruction {
    pub fn from_str(_line: &str) -> Result<Instruction> {
        Err("bad".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_lines() {
        let line = "b inc 5 if a > 1";
        let result = Instruction::from_str(line).unwrap();
        assert_eq!(
            result,
            Instruction {
                register: "b".to_string(),
                instruction: InstructionType::Increment,
                condition: Condition {
                    register: "a".to_string(),
                    operation: Operation::GreaterThan,
                    value: 1,
                },
            }
        );
    }
}
