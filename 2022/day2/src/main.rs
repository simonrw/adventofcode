use std::{
    io::{BufRead, BufReader, Cursor},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum PlayOption {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for PlayOption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(PlayOption::Rock),
            "B" | "Y" => Ok(PlayOption::Paper),
            "C" | "Z" => Ok(PlayOption::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Play {
    you: PlayOption,
    opponent: PlayOption,
}

impl Play {
    fn from_str(line: &str) -> Play {
        let mut words = line.trim().split_whitespace();
        let their_play = words.next().unwrap().parse().unwrap();
        let your_play = words.next().unwrap().parse().unwrap();
        Play {
            you: your_play,
            opponent: their_play,
        }
    }

    fn score(&self) -> i64 {
        let my_shape_score = match self.you {
            PlayOption::Rock => 1,
            PlayOption::Paper => 2,
            PlayOption::Scissors => 3,
        };

        let outcome_score = self.compute_outcome_score();
        my_shape_score + outcome_score
    }

    fn compute_outcome_score(&self) -> i64 {
        match (self.you, self.opponent) {
            // loss
            (PlayOption::Rock, PlayOption::Paper)
            | (PlayOption::Paper, PlayOption::Scissors)
            | (PlayOption::Scissors, PlayOption::Rock) => 0,

            // draw
            (PlayOption::Rock, PlayOption::Rock)
            | (PlayOption::Paper, PlayOption::Paper)
            | (PlayOption::Scissors, PlayOption::Scissors) => 3,

            // victory
            (PlayOption::Rock, PlayOption::Scissors)
            | (PlayOption::Paper, PlayOption::Rock)
            | (PlayOption::Scissors, PlayOption::Paper) => 6,
        }
    }
}

fn parse_input(s: &str) -> Vec<Play> {
    let mut out = Vec::new();

    let b = BufReader::new(Cursor::new(s));
    for line in b.lines() {
        let line = line.unwrap();
        let play = Play::from_str(&line);
        out.push(play);
    }

    out
}

fn run(input: &str) -> i64 {
    let plays = parse_input(input);
    let score: i64 = plays.iter().map(Play::score).sum();
    score
}

fn main() {
    let input = include_str!("../input");
    println!("score {}", run(input));
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn example() {
        let input = "A Y
            B X
            C Z";

        assert_eq!(run(input), 15);
    }
}
