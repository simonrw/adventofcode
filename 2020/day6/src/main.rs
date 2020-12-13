use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let part_one_total = group_sum(&input);
    println!("{}", part_one_total);

    let part_two_total = all_group_sum(&input);
    println!("{}", part_two_total);
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

fn all_group_sum(input: &str) -> usize {
    let mut count = 0;
    let mut group_total: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            count += finish_group(&group_total);
            group_total.clear();
        } else {
            let group: Vec<_> = line.chars().collect();
            group_total.push(group);
        }
    }

    count += finish_group(&group_total);

    count
}

fn finish_group(group_total: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let mut values = HashSet::new();
    for e in group_total {
        for every in e {
            values.insert(every);
        }
    }

    for every in values {
        let mut found = true;
        for person in group_total {
            if !person.contains(every) {
                found = false;
                break;
            }
        }

        if found {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_example_part_one() {
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

    #[test]
    fn given_example_part_two() {
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

        assert_eq!(all_group_sum(&input), 6);
    }
}
