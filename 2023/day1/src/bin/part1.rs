fn main() {
    let input = include_str!("../../input.txt");

    let mut total = 0;
    for line in input.lines() {
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
        total += sum;
    }
    println!("{total}");
}
