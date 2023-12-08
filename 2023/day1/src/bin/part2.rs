type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

trait ToDigit {
    fn to_digit(&self) -> Result<u8>;
}

impl ToDigit for &str {
    fn to_digit(&self) -> Result<u8> {
        match *self {
            "one" => Ok(1),
            "two" => Ok(2),
            "three" => Ok(3),
            "four" => Ok(4),
            "five" => Ok(5),
            "six" => Ok(6),
            "seven" => Ok(7),
            "eight" => Ok(8),
            "nine" => Ok(9),
            _ => Err("invalid digit".into()),
        }
    }
}

fn process<'a>(lines: impl Iterator<Item = &'a str>) -> usize {
    let mut total = 0usize;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("#") {
            continue;
        }

        let value = calculate_from_line(line);
        total += value as usize;
    }
    total as _
}

fn calculate_from_line(line: &str) -> usize {
    let mut digits = Vec::new();
    let mut line = &line[..];
    while !line.is_empty() {
        // check words first
        for word in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ] {
            if line.starts_with(word) {
                digits.push(word.to_digit().unwrap());
                line = &line[1..];
                break;
            }
        }

        if line.is_empty() {
            break;
        }

        if let Ok(digit) = line[0..1].parse::<u8>() {
            digits.push(digit);
            line = &line[1..];
            continue;
        }

        if line.is_empty() {
            break;
        }

        line = &line[1..];
    }


    let (first, last) = match digits.len() {
        0 => unreachable!(),
        1 => {
            let first = digits[0];
            let last = digits[0];
            (first, last)
        }
        _ => {
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            (*first, *last)
        }
    };

    let value = first * 10 + last;
    value as usize
}

fn main() {
    let input = include_str!("../../input.txt");

    let total = process(input.lines());
    println!("{total}");
}

#[cfg(test)]
mod tests {
    use crate::{calculate_from_line, process};

    #[test]
    fn example_lines() {
        let lines = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
oneight
";
        let values: Vec<_> = lines.lines().map(calculate_from_line).collect();
        assert_eq!(values, [29, 83, 13, 24, 42, 14, 76, 18]);
    }

    #[test]
    fn example_input() {
        let lines = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
        .lines();
        assert_eq!(process(lines), 281);
    }
}
