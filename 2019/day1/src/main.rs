use std::fs;

fn fuel_for(mass: &i64) -> i64 {
    (((*mass as f64) / 3.).floor() - 2.) as _
}

fn main() {
    let input_data_text = fs::read_to_string("input.txt").unwrap();
    let input_data: Vec<_> = input_data_text
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect();
    // Total fuel
    println!(
        "Total fuel required: {}",
        input_data.iter().map(fuel_for).sum::<i64>()
    );
}

#[cfg(test)]
mod tests {
    use super::fuel_for;

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
}
