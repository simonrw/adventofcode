extern crate rayon;

use std::error::Error;
type Result<T> = ::std::result::Result<T, Box<Error>>;

use std::str::FromStr;
use rayon::prelude::*;

pub fn puzzle(input: &[u8], n_addresses: u32) -> Result<u32> {
    let mut ip_ranges: Vec<IpRange> = Vec::new();
    for line in std::str::from_utf8(input)?.lines() {
        ip_ranges.push(line.parse()?);
    }

    let mut n_found = 0;
    Ok((0..n_addresses).into_par_iter().filter(|&ip_addr| {
        let mut found = false;
        for range in ip_ranges.iter() {
            if range.contains(ip_addr) {
                found = true;
                break;
            }

        }
        !found
    }).count() as u32)
}

#[derive(Debug, PartialEq, Eq)]
pub struct IpRange {
    start: u32,
    end: u32,
}

impl IpRange {
    fn contains(&self, ip_address: u32) -> bool {
        return ip_address >= self.start && ip_address <= self.end;
    }
}

impl FromStr for IpRange {
    type Err = Box<Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts: Vec<_> = s.split("-").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid input string: {}", s).into());
        }

        Ok(IpRange {
            start: parts[0].parse()?,
            end: parts[1].parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_ip_range() {
        let text = "2365712272-2390766206";
        assert_eq!(text.parse::<IpRange>().unwrap(),
                   IpRange {
                       start: 2365712272,
                       end: 2390766206,
                   });
    }

    #[test]
    fn invalid_format() {
        let text = "2365712272";
        assert!(text.parse::<IpRange>().is_err());
    }

    #[test]
    fn invalid_number_values() {
        let text = "2365712272-foobar";
        assert!(text.parse::<IpRange>().is_err());
    }

    #[test]
    fn contains() {
        let range = IpRange {
            start: 5,
            end: 10,
        };
        for i in 0..5 {
            assert!(!range.contains(i));
        }

        for i in 5..11 {
            assert!(range.contains(i));
        }

        for i in 11..15 {
            assert!(!range.contains(i));
        }
    }
}
