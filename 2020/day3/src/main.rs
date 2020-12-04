use eyre::{bail, Result};

#[derive(Debug)]
enum Token {
    Blank,
    Tree,
}

#[derive(Debug)]
struct Board {
    tokens: Vec<Token>,
    width: usize,
}

impl Board {
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = Vec::new();
        let mut width = 0;

        for (i, c) in s.chars().enumerate() {
            if c == '\n' {
                if width == 0 {
                    // We have not seen a newline before, so now
                    // we know the width of the board.
                    width = i;
                }

                continue;
            }

            if c == '.' {
                tokens.push(Token::Blank);
            } else if c == '#' {
                tokens.push(Token::Tree);
            } else {
                bail!("unknown token: {:?}", c);
            }
        }

        Ok(Self { tokens, width })
    }
}

fn main() {
    let input = include_str!("../input");
    let board = Board::from_str(input);
    println!("{:?}", board);
}
