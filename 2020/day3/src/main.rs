use eyre::{bail, Result};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Token {
    Blank,
    Tree,
}

#[derive(Debug, Clone, Copy)]
struct Width(u64);

impl std::ops::Rem<Width> for u64 {
    type Output = u64;

    fn rem(self, rhs: Width) -> Self::Output {
        self % rhs.0
    }
}

impl std::ops::Mul<Width> for u64 {
    type Output = u64;

    fn mul(self, rhs: Width) -> Self::Output {
        self * rhs.0
    }
}

#[derive(Debug)]
struct Board {
    tokens: Vec<Token>,
    width: Width,
    lines: u64,
}

impl Board {
    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = Vec::new();
        let mut width = 0;
        let mut lines = 0;

        for (i, c) in s.chars().enumerate() {
            if c == '\n' {
                if width == 0 {
                    // We have not seen a newline before, so now
                    // we know the width of the board.
                    width = i as u64;
                }

                lines += 1;

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

        Ok(Self {
            tokens,
            width: Width(width),
            lines,
        })
    }

    fn token_at(&self, point: &Point) -> Token {
        let idx = point.y * self.width + point.x;
        self.tokens[idx as usize]
    }

    fn run(&mut self) -> Result<HashMap<&(u64, u64), u64>> {
        let mut results = HashMap::new();
        for offsets in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            let mut init = Point::init();
            let mut trees = 0;
            while init.y < self.lines {
                if let Token::Tree = self.token_at(&init) {
                    trees += 1;
                }

                init = init.next(offsets.0, offsets.1, self.width);
            }
            results.insert(offsets, trees);
        }

        Ok(results)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn init() -> Self {
        Self { x: 0, y: 0 }
    }

    fn next(self, x: u64, y: u64, width: Width) -> Point {
        Point {
            x: (self.x + x) % width,
            y: self.y + y,
        }
    }
}

fn main() -> Result<()> {
    let input = include_str!("../input");
    let mut board = Board::from_str(input)?;

    let n_trees = board.run()?;
    println!("Found {:?} trees", n_trees);

    let result = n_trees.values().fold(1, |acc, v| acc * v);
    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_point() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = p1.next(100);
        assert_eq!(p2, Point { x: 3, y: 1 });
        let p3 = p2.next(100);
        assert_eq!(p3, Point { x: 6, y: 2 });
    }

    #[test]
    fn wraparound() {
        let p1 = Point { x: 4, y: 0 };
        let p2 = p1.next(6);
        assert_eq!(p2, Point { x: 1, y: 1 });
    }
}
