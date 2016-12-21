use std::str::FromStr;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Instruction {
    direction: Direction,
    distance: usize,
}

impl FromStr for Instruction {
    type Err = Box<Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut chars = s.chars();

        match chars.next() {
            Some('L') => {
                let dist_str: String = chars.filter(|&c| c != ',').collect();
                let distance = dist_str.parse()?;
                Ok(Instruction {
                    direction: Direction::Left,
                    distance: distance,
                })
            },
            Some('R') => {
                let dist_str: String = chars.filter(|&c| c != ',').collect();
                let distance = dist_str.parse()?;
                Ok(Instruction {
                    direction: Direction::Right,
                    distance: distance,
                })
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Facing {
    North,
    South,
    East,
    West,
}

fn next_facing(facing: Facing, direction: Direction) -> Facing {
    use Direction::*;
    use Facing::*;

    match facing {
        North => if direction == Left { West } else { East },
        South => if direction == Left { East } else { West },
        West => if direction == Left { South } else { North },
        East => if direction == Left { North } else { South },
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

pub fn puzzle(input: &[u8]) -> Result<i32> {
    use Facing::*;

    let instructions = std::str::from_utf8(input)?.trim();
    let words = instructions.split_whitespace();
    let instructions = words.map(Instruction::from_str);

    let mut pos = Position::default();
    let mut facing = North;
    let mut history = Vec::new();

    for (i, instruction) in instructions.enumerate() {
        if let Ok(instruction) = instruction {
            facing = next_facing(facing, instruction.direction);
            let distance = instruction.distance;

            match facing {
                North => pos.y += distance as i32,
                South => pos.y -= distance as i32,
                East => pos.x += distance as i32,
                West => pos.x -= distance as i32,
            }

            if history.contains(&pos) {
                println!("{} => Position {:?} already found at distance {}", i, pos, pos.distance());
            }

            history.push(pos);
        }
    }

    Ok(pos.distance())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_facing() {
        assert_eq!(next_facing(Facing::North, Direction::Left), Facing::West);
    }

    #[test]
    fn first_example() {
        let input = "R2, L3\n";
        assert_eq!(puzzle(input.as_bytes()).unwrap(), 5);
    }

    #[test]
    fn second_example() {
        let input = "R2, R2, R2\n";
        assert_eq!(puzzle(input.as_bytes()).unwrap(), 2);
    }

    #[test]
    fn third_example() {
        let input = "R5, L5, R5, R3";
        assert_eq!(puzzle(input.as_bytes()).unwrap(), 12);
    }
}
