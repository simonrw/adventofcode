use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Debug, Eq, PartialEq)]
pub enum InstructionType {
    Increment,
    Decrement,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Condition {
    register: String,
    operation: Operation,
    value: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Operation {
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    Equal,
    NotEqual,
}

impl FromStr for Operation {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            ">" => Ok(Operation::GreaterThan),
            "<" => Ok(Operation::LessThan),
            ">=" => Ok(Operation::GreaterEqual),
            "<=" => Ok(Operation::LessEqual),
            "==" => Ok(Operation::Equal),
            "!=" => Ok(Operation::NotEqual),
            other => Err(format!("invalid evaluation type: {}", other).into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    register: String,
    instruction: InstructionType,
    value: i32,
    condition: Condition,
}

impl FromStr for Instruction {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let main_register = parts[0].trim();
        let instruction_type = InstructionType::from_str(&parts[1])?;
        let value: i32 = parts[2].parse()?;
        let conditional_register = parts[4];
        let operation = Operation::from_str(parts[5])?;
        let conditional_value: i32 = parts[6].parse()?;

        Ok(Instruction {
            register: main_register.to_string(),
            instruction: instruction_type,
            value: value,
            condition: Condition {
                register: conditional_register.to_string(),
                operation: operation,
                value: conditional_value,
            },
        })
    }
}

impl FromStr for InstructionType {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "inc" => Ok(InstructionType::Increment),
            "dec" => Ok(InstructionType::Decrement),
            other => Err(format!("invalid instruction type: {}", other).into()),
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
        ::std::process::exit(1);
    }
}

pub fn run() -> Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();
        let instruction = Instruction::from_str(&line)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction_type() {
        let result: InstructionType = "inc".parse().unwrap();
        assert_eq!(result, InstructionType::Increment);

        let result: InstructionType = "dec".parse().unwrap();
        assert_eq!(result, InstructionType::Decrement);
    }

    #[test]
    fn parsing_lines() {
        let line = "b inc 5 if a > 1";
        let result = Instruction::from_str(line).unwrap();
        assert_eq!(
            result,
            Instruction {
                register: "b".to_string(),
                instruction: InstructionType::Increment,
                value: 5,
                condition: Condition {
                    register: "a".to_string(),
                    operation: Operation::GreaterThan,
                    value: 1,
                },
            }
        );
    }
}
