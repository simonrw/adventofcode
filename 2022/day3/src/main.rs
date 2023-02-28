use std::collections::HashSet;

#[derive(Debug)]
struct Backpack<'s> {
    left: &'s str,
    right: &'s str,
}

fn score_letter(letter: char) -> u64 {
    (if letter.is_uppercase() {
        (letter as i32) - ('a' as i32) + 59
    } else {
        (letter as i32) - ('a' as i32) + 1
    }) as _
}

impl<'s> Backpack<'s> {
    pub fn score(&self) -> u64 {
        let common_item = self.common_item();
        score_letter(common_item)
    }

    fn common_item(&self) -> char {
        let left_items: HashSet<char> = self.left.chars().collect();
        let right_items: HashSet<char> = self.right.chars().collect();
        let intersection: Vec<_> = left_items.intersection(&right_items).collect();
        if intersection.len() != 1 {
            panic!("unusual number of common items");
        }
        *intersection.into_iter().next().unwrap()
    }
}

fn parse_line(line: &str) -> Backpack<'_> {
    let line_length = line.trim().len();
    if line_length % 2 != 0 {
        panic!("odd line length");
    }

    let (l, r) = line.split_at(line_length / 2);
    Backpack { left: l, right: r }
}

fn compute_group_score(lines: &[&str]) -> u64 {
    assert_eq!(lines.len(), 3);
    let hashsets = lines.iter().map(|l| l.chars().collect::<HashSet<char>>());
    let group_letter_set = hashsets
        .reduce(|acc, e| acc.intersection(&e).cloned().collect())
        .unwrap();
    assert_eq!(group_letter_set.len(), 1);
    score_letter(*group_letter_set.iter().next().unwrap())
}

fn part1<'a>(lines: impl Iterator<Item = &'a str>) {
    let mut sum = 0;
    for line in lines {
        let backpack = parse_line(line);
        let score = backpack.score();
        sum += score;
    }
    println!("part 1: {sum}");
}

fn part2<'a>(input: &str) {
    let grouped = input
        .split_whitespace()
        .step_by(3)
        .zip(input.split_whitespace().skip(1).step_by(3))
        .zip(input.split_whitespace().skip(2).step_by(3))
        .map(|((a, b), c)| [a, b, c]);
    let mut total = 0;
    for group in grouped {
        let score = compute_group_score(&group);
        total += score;
    }
    println!("part 2: {total}");
}

fn main() {
    let input = include_str!("../input");
    part1(input.split_whitespace());
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_score() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let backpack = parse_line(line);
        assert_eq!(backpack.score(), 16);
    }

    #[test]
    fn lines_scores() {
        let lines = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let scores = [16, 38, 42, 22, 20, 19];
        for (i, line) in lines.split_whitespace().enumerate() {
            let line = line.trim();
            let backpack = parse_line(&line);
            assert_eq!(backpack.score(), scores[i]);
        }
    }

    #[test]
    fn group_score() {
        let lines = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        assert_eq!(compute_group_score(&lines), 18);

        let lines = [
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        assert_eq!(compute_group_score(&lines), 52);
    }
}
