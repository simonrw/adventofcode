use std::error::Error;
type Result<T> = ::std::result::Result<T, Box<Error>>;

use std::str::FromStr;

pub fn puzzle(input: &[u8]) -> Result<u32> {
    Ok(32)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Manipulation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    Rotate(Direction, usize),
    RotateAround(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Manipulation {
    type Err = Box<Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let first_word = words.next();
        match first_word {
            Some("swap") => {
                let rest: Vec<&str> = words.collect();
                match rest[0] {
                    "letter" => {
                        let start = rest[1].chars().next().unwrap();
                        let end = rest[4].chars().next().unwrap();
                        Ok(Manipulation::SwapLetter(start, end))
                    },
                    "position" => {
                        let start = rest[1].parse().unwrap();
                        let end = rest[4].parse().unwrap();
                        Ok(Manipulation::SwapPosition(start, end))
                    },
                    _ => unimplemented!(),
                }
            },
            Some("rotate") => {
                let rest: Vec<&str> = words.collect();
                match rest[0] {
                    "left" => {
                        let distance = rest[1].parse()?;
                        Ok(Manipulation::Rotate(Direction::Left, distance))
                    },
                    "right" => {
                        let distance = rest[1].parse()?;
                        Ok(Manipulation::Rotate(Direction::Right, distance))
                    },
                    "based" => {
                        let letter = rest[5].chars().next().unwrap();
                        Ok(Manipulation::RotateAround(letter))
                    },
                    _ => unimplemented!(),
                }
            },
            Some("reverse") => {
                let rest: Vec<&str> = words.collect();
                let start = rest[1].parse()?;
                let end = rest[3].parse()?;

                Ok(Manipulation::Reverse(start, end))
            },
            Some("move") => {
                let rest: Vec<&str> = words.collect();
                let src = rest[1].parse()?;
                let dest = rest[4].parse()?;

                Ok(Manipulation::Move(src, dest))
            },
            None => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

pub fn transform(start: &str, manipulations: &[Manipulation]) -> Result<String> {
    use Manipulation::*;

    let mut result = start.to_string();

    for m in manipulations {
        match *m {
            SwapLetter(src, dest) => {
                result = result.chars().map(|c| {
                    if c == src {
                        dest
                    } else {
                        c
                    }
                }).collect()
            },
            SwapPosition(src, dest) => {

            },
            _ => unimplemented!(),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const start: &'static str = "abcdefgh";

    #[test]
    fn parse_swap_position() {
        let input = "swap position 1 with position 3";
        assert_eq!(input.parse::<Manipulation>().unwrap(), Manipulation::SwapPosition(1, 3));
    }

    #[test]
    fn parse_swap_letter() {
        let input = "swap letter b with letter e";
        assert_eq!(input.parse::<Manipulation>().unwrap(), Manipulation::SwapLetter('b', 'e'));
    }

    #[test] 
    fn parse_rotate() {
        let input = "rotate right 4 steps";
        assert_eq!(input.parse::<Manipulation>().unwrap(), Manipulation::Rotate(Direction::Right, 4));
    }

    #[test] 
    fn parse_reverse() {
        let input = "reverse positions 0 through 4";
        assert_eq!(input.parse::<Manipulation>().unwrap(), Manipulation::Reverse(0, 4));
    }

    #[test] 
    fn parse_move() {
        let input = "move position 3 to position 2";
        assert_eq!(input.parse::<Manipulation>().unwrap(), Manipulation::Move(3, 2));
    }

    #[test] 
    fn parse_rotate_around() {
        let input = "rotate based on position of letter e";
        assert_eq!(input.parse::<Manipulation>().unwrap(), Manipulation::RotateAround('e'));
    }

    #[test]
    fn parse_all_input() {
        use std::fs::File;
        use std::io::Read;

        let input_path = "input";
        let mut input = Vec::new();
        let mut file = File::open(input_path).expect(&format!("Could not open {}", input_path));
        file.read_to_end(&mut input).expect("Could not read from file");
        let input_str = std::str::from_utf8(&input).unwrap();

        for line in input_str.lines() {
            match line.parse::<Manipulation>() {
                Ok(_) => {},
                Err(e) => panic!("Error in line {}: {}", line, e),
            }
        }
    }

    #[test]
    fn transform_swap_letters() {
        let manipulations = vec![Manipulation::SwapLetter('b', 'e')];
        assert_eq!(transform(&start, &manipulations).unwrap(), "aecdefgh".to_string());
    }

    #[test]
    fn transform_swap_positions() {
        let manipulations = vec![Manipulation::SwapPosition(1, 3)];
        assert_eq!(transform(&start, &manipulations).unwrap(), "adcbefgh".to_string());
    }
}
