fn main() {
    println!("Hello, world!");
}

fn day12_part1(text: &str) -> usize {
    for line in text.lines() {
        let parsed: Mapping = line.parse().unwrap();
    }

    6
}

#[derive(Debug, PartialEq, Eq)]
pub struct Mapping {
    src: usize,
    dests: Vec<usize>,
}

impl std::str::FromStr for Mapping {
    type Err = Box<std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let src: usize = parts[0].parse().unwrap();
        assert_eq!(parts[1], "<->");
        let dests = parts.into_iter().skip(2).map(|text| text.replace(",", "").parse().unwrap()).collect();

        Ok(Mapping {
            src: src, dests: dests,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_example() {
        let example = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        let result = day12_part1(&example);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_parse_simple_line() {
        let line = "0 <-> 2";
        let result = line.parse::<Mapping>().unwrap();
        assert_eq!(result, Mapping {
            src: 0,
            dests: vec![2],
        });
    }

    #[test]
    fn test_parse_complex_line() {
        let line = "4 <-> 2, 3, 6";
        let result = line.parse::<Mapping>().unwrap();
        assert_eq!(result, Mapping {
            src: 4,
            dests: vec![2, 3, 6],
        });
    }
}
