use eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::digit1,
    combinator::complete,
    AsChar, IResult,
};
use std::collections::HashMap;
use std::str::Lines;

enum Height {
    Centimeters(u64),
    Inches(u64),
}

#[derive(Debug)]
struct Passport<'a> {
    entries: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn is_valid(&self) -> bool {
        let required_keys = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        for key in required_keys {
            if !self.entries.contains_key(key) {
                return false;
            }
        }

        self.birth_year_valid()
            && self.issue_year_valid()
            && self.expiration_year_valid()
            && self.height_valid()
            && self.hair_color_valid()
            && self.eye_color_valid()
            && self.passport_id_valid()
    }

    fn birth_year_valid(&self) -> bool {
        self.year_in_range("byr", 1920, 2002)
    }

    fn issue_year_valid(&self) -> bool {
        self.year_in_range("iyr", 2010, 2020)
    }

    fn expiration_year_valid(&self) -> bool {
        self.year_in_range("eyr", 2020, 2030)
    }

    fn height_valid(&self) -> bool {
        self.parse_height(self.entries["hgt"])
            .map(|(_, h)| match h {
                Height::Centimeters(h) => h >= 150 && h <= 193,
                Height::Inches(h) => h >= 59 && h <= 76,
            })
            .unwrap_or(false)
    }

    fn hair_color_valid(&self) -> bool {
        self.parse_hair_color(self.entries["hcl"])
            .map(|_| true)
            .unwrap_or(false)
    }

    fn parse_hair_color<'b>(&self, s: &'b str) -> IResult<&'b str, ()> {
        let (s, _) = tag("#")(s)?;
        let (s, _) = take_while_m_n(6, 6, |c: char| c.is_hex_digit())(s)?;
        Ok((s, ()))
    }

    fn eye_color_valid(&self) -> bool {
        let c = self.entries["ecl"];
        match c {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }

    fn passport_id_valid(&self) -> bool {
        let pid_str = self.entries["pid"];
        if pid_str.len() != 9 {
            return false;
        }

        pid_str.parse::<i64>().map(|_| true).unwrap_or(false)
    }

    fn parse_height<'b>(&self, s: &'b str) -> IResult<&'b str, Height> {
        let (s, num) = digit1(s)?;
        let (s, unit) = alt((tag("cm"), tag("in")))(s)?;

        if unit == "cm" {
            Ok((s, Height::Centimeters(num.parse().unwrap())))
        } else if unit == "in" {
            Ok((s, Height::Inches(num.parse().unwrap())))
        } else {
            Err(nom::Err::Failure(nom::error::make_error(
                s,
                nom::error::ErrorKind::Tag,
            )))
        }
    }

    fn year_in_range(&self, key: &str, lower: i32, upper: i32) -> bool {
        match digit1::<_, ()>(self.entries[key]) {
            Ok((_, yr)) => {
                let yr = yr.parse::<i32>().unwrap();
                yr >= lower && yr <= upper
            }
            _ => false,
        }
    }
}

struct PassportIter<'a> {
    lines: Lines<'a>,
    complete: bool,
}

impl<'a> PassportIter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            lines: s.lines(),
            complete: false,
        }
    }
}

impl<'a> std::iter::Iterator for PassportIter<'a> {
    type Item = Passport<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entries = HashMap::new();
        loop {
            if self.complete {
                return None;
            }

            match self.lines.next() {
                Some(line) => {
                    let line = line.trim();

                    if line.is_empty() {
                        break;
                    }

                    for word in line.split_whitespace() {
                        let parts: Vec<_> = word.split(":").collect();
                        assert_eq!(parts.len(), 2);

                        entries.insert(parts[0], parts[1]);
                    }
                }
                None => {
                    self.complete = true;
                    return Some(Passport { entries });
                }
            }
        }
        Some(Passport { entries })
    }
}

trait ByPassport {
    fn by_passport(&self) -> PassportIter;
}

impl ByPassport for &str {
    fn by_passport(&self) -> PassportIter {
        PassportIter::new(self)
    }
}

fn main() {
    let input = include_str!("../input");

    let mut n_valid = 0;
    let mut n_passports = 0;
    for passport in input.by_passport() {
        n_passports += 1;
        if passport.is_valid() {
            n_valid += 1;
        }
    }

    println!("{} valid out of {}", n_valid, n_passports);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference() {
        let reference = "
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
        ";

        let mut n_valid = 0;
        for passport in reference.by_passport() {
            if passport.is_valid() {
                n_valid += 1;
            }
        }

        assert_eq!(n_valid, 2);
    }
}
