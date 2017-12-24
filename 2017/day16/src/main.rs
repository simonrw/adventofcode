#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    label: char,
}

impl Move {
    pub fn parse(c: char) -> Self {
        Move { label: c }
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Lineup {
    moves: Vec<Move>,
}

impl<'a> Lineup {
    pub fn from_str(input: &str) -> Self {
        Lineup { moves: input.chars().map(Move::parse).collect() }
    }

    pub fn to_str(self) -> String {
        self.moves.into_iter().map(|m| m.label).collect::<String>()
    }

    pub fn transform(self, _instructions: &str) -> Self {
        self
    }
}

pub fn day17_part1(input: &str) -> Lineup {
    let input = Lineup::from_str(input);
    input.transform("")
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "abcde";
        let result = day17_part1(input);
        assert_eq!(result.to_str(), "baedc".to_string());
    }

    #[test]
    fn test_parse() {
        let input = "abcde";
        let result = Lineup::from_str(input);
        assert_eq!(
            result,
            Lineup {
                moves: vec![
                    Move { label: 'a' },
                    Move { label: 'b' },
                    Move { label: 'c' },
                    Move { label: 'd' },
                    Move { label: 'e' },
                ],
            }
        );
    }
}
