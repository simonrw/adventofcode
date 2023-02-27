use std::collections::HashSet;

#[derive(Debug)]
struct Backpack<'s> {
    left: &'s str,
    right: &'s str,
}

impl<'s> Backpack<'s> {
    pub fn score(&self) -> u64 {
        let common_item = self.common_item();
        (if common_item.is_uppercase() {
            (common_item as i32) - ('a' as i32) + 59
        } else {
            (common_item as i32) - ('a' as i32) + 1
        }) as _
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

fn main() {
    let input = include_str!("../input");
    let lines = input.split_whitespace();
    let mut sum = 0;
    for line in lines {
        let backpack = parse_line(&line);
        let score = backpack.score();
        sum += score;
    }
    println!("part 1: {sum}");
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
}
