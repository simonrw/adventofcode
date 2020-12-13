use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let part_one_total = group_sum(&input);
    println!("{}", part_one_total);
}

fn group_sum(input: &str) -> usize {
    let mut count = 0;
    let mut group_items = HashSet::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            count += group_items.len();
            group_items.clear();
        } else {
            for c in line.chars() {
                group_items.insert(c);
            }
        }
    }

    // Add the final group
    count += group_items.len();

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b
";

        assert_eq!(group_sum(&input), 11);
    }
}
