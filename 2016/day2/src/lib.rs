use std::str::FromStr;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<Error>>;

pub fn puzzle(input: &[u8]) -> Result<i32> {
    Ok(0)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Instruction::*;

        let mut chars = s.chars();
        let c = chars.next();
        Ok(match c {
            Some('U') => Up,
            Some('D') => Down,
            Some('L') => Left,
            Some('R') => Right,
            None => {
                return Err(format!("no character found").into());
            },
            _ => {
                return Err(format!("invalid parse: {}", c.unwrap()).into());
            },
        })
    }
}

pub fn button_from(start: u8, instructions: &[Instruction]) -> Option<u8> {
    Some(1)
}

pub fn parse_instructions(s: &str) -> Result<Vec<Instruction>> {
    s.chars().map(Instruction::from_str).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_up() {
        use Instruction::*;
        assert_eq!("U".parse::<Instruction>().unwrap(), Up);
    }

    #[test]
    fn parse_down() {
        use Instruction::*;
        assert_eq!("D".parse::<Instruction>().unwrap(), Down);
    }

    #[test]
    fn parse_left() {
        use Instruction::*;
        assert_eq!("L".parse::<Instruction>().unwrap(), Left);
    }

    #[test]
    fn parse_right() {
        use Instruction::*;
        assert_eq!("R".parse::<Instruction>().unwrap(), Right);
    }

    #[test]
    fn test_parse_instructions(){
        use Instruction::*;

        let instructions = parse_instructions("ULL").unwrap();
        assert_eq!(instructions, vec![Up, Left, Left]);
    }

    // #[test]
    // fn part1() {
    //     let instructions = parse_instructions("ULL").unwrap();
    //     assert_eq!(button_from(5, &instructions), Some(1));
    // }
}
