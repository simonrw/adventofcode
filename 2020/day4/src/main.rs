use std::collections::HashMap;
use std::str::Lines;

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

        return true;
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
