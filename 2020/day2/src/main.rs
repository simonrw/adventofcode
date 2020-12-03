// 17-19 p: pwpzpfbrcpppjppbmppp

use nom::IResult;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit0, space0},
};

fn count_range(s: &str) -> IResult<&str, Range> {
    let (s, beginning) = digit0(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = tag("-")(s)?;
    let (s, _) = space0(s)?;
    let (s, end) = digit0(s)?;

    // Unwraps ok as we know that the strings are digits

    Ok((
        s,
        Range {
            lower: beginning.parse().unwrap(),
            upper: end.parse().unwrap(),
        },
    ))
}

#[derive(Debug)]
struct Range {
    lower: u32,
    upper: u32,
}

impl Range {
    fn includes(&self, value: u32) -> bool {
        value >= self.lower && value <= self.upper
    }
}

#[derive(Debug)]
struct Entry<'a> {
    range: Range,
    c: char,
    password: &'a str,
}

impl<'a> Entry<'a> {
    fn passes(&self) -> bool {
        let cs: Vec<_> = self.password.chars().collect();
        let first_char = cs[self.range.lower as usize - 1];
        let second_char = cs[self.range.upper as usize - 1];

        match (first_char == self.c, second_char == self.c) {
            (true, true) | (false, false) => false,
            _ => true,
        }
    }

    // fn passes(&self) -> bool {
    //     let count = self.password.chars().filter(|c| *c == self.c).count();
    //     self.range.includes(count as u32)
    // }
}

fn parse_entry(s: &str) -> IResult<&str, Entry<'_>> {
    let (s, range) = count_range(s)?;
    let (s, _) = space0(s)?;
    let (s, c) = alpha1(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = space0(s)?;
    let (s, password) = alpha1(s)?;

    assert!(range.lower < range.upper);

    Ok((
        s,
        Entry {
            range,
            c: c.chars().next().unwrap(),
            password,
        },
    ))
}

fn main() {
    let input = include_str!("../input");

    let mut count = 0;
    for line in input.lines() {
        match nom::combinator::complete(parse_entry)(line) {
            Ok((_, entry)) => {
                if entry.passes() {
                    count += 1;
                }
            }
            Err(e) => eprintln!("parse error: {:?}", e),
        }
    }

    println!("Number of passwords that pass: {}", count);
}
