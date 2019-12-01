use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn fuel_for(mass: &i64) -> i64 {
    (((*mass as f64) / 3.).floor() - 2.) as _
}

fn total_fuel_for(mass: &i64, accum: i64) -> i64 {
    if *mass <= 0 {
        return accum;
    }

    let fuel = fuel_for(mass);

    total_fuel_for(&fuel, accum + *mass)
}

pub fn compute_total_fuel_for(mass: &i64) -> i64 {
    total_fuel_for(mass, 0) - mass
}

fn read_input_data<S>(fname: S) -> Result<Vec<i64>>
where
    S: AsRef<Path>,
{
    let input_data_text = fs::read_to_string(fname)?;
    let input_data: Vec<_> = input_data_text
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect();
    Ok(input_data)
}

fn part1() {
    let input_data = read_input_data("input.txt").unwrap();
    // Total fuel
    println!(
        "Total fuel required: {}",
        input_data.iter().map(fuel_for).sum::<i64>()
    );
}

fn part2() {
    let input_data = read_input_data("input.txt").unwrap();
    // Total fuel
    println!(
        "Total fuel required: {}",
        input_data
            .iter()
            .map(|v| compute_total_fuel_for(v))
            .sum::<i64>()
    );
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::{compute_total_fuel_for, fuel_for};

    #[test]
    fn example1() {
        let mass = 12;
        assert_eq!(fuel_for(&mass), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(fuel_for(&14), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(fuel_for(&1969), 654);
    }

    #[test]
    fn example4() {
        assert_eq!(fuel_for(&100756), 33583);
    }

    // Part 2 tests

    #[test]
    fn example5() {
        assert_eq!(compute_total_fuel_for(&14), 2);
    }

    #[test]
    fn example6() {
        assert_eq!(compute_total_fuel_for(&1969), 966);
    }

    #[test]
    fn example7() {
        assert_eq!(compute_total_fuel_for(&100756), 50346);
    }
}
