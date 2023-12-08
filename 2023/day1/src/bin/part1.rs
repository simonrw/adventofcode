fn calculate_from_line(line: &str) -> usize {
    let digits: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();
    let (first, last) = if digits.len() == 0 {
        todo!()
    } else if digits.len() == 1 {
        let d = digits.first().unwrap().to_digit(10).unwrap();
        (d, d)
    } else {
        let first = digits.first().unwrap().to_digit(10).unwrap();
        let last = digits.last().unwrap().to_digit(10).unwrap();
        (first, last)
    };
    let sum = first * 10 + last;
    sum as usize
}

fn process(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        total += calculate_from_line(&line);
    }
    total
}

fn main() {
    let input = include_str!("../../input.txt");
    let total = process(input);
    println!("{total}");
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]

    fn example_day1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process(input), 142);
    }
}
