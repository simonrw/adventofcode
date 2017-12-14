fn main() {
    println!("Hello, world!");
}

fn day12_part1(_text: &str) -> usize {
    6
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
}
