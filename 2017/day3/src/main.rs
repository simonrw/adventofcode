fn compute_width(_x: i32) -> i32 {
    0
}

pub fn compute_distance(_loc: i32) -> i32 {
    0
}

fn main() {
    let location = 277678;
    println!(
        "DISTANCE FOR LOCATION {}: {}",
        location,
        compute_distance(location)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_1() {
        assert_eq!(compute_width(1), 0);
    }

    #[test]
    fn width_2() {
        assert_eq!(compute_width(2), 1);
    }

    #[test]
    fn width_15() {
        assert_eq!(compute_width(15), 0);
    }

    #[test]
    fn test_1() {
        assert_eq!(compute_distance(1), 0);
    }

    #[test]
    fn test_12() {
        assert_eq!(compute_distance(12), 3);
    }

    #[test]
    fn test_23() {
        assert_eq!(compute_distance(23), 2);
    }
}
