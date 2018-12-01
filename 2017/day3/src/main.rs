fn higher_square(loc: i32) -> (i32, i32) {
    let n = (loc as f32).sqrt() as i32 + 1;
    (n * n, n)
}

pub fn compute_distance(loc: i32) -> i32 {
    let (ns, n) = higher_square(loc);
    println!("{} {}", ns, n);
    100
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
    fn test_first() {
        assert_eq!(compute_distance(1), 0);
    }

    #[test]
    fn test_second() {
        assert_eq!(compute_distance(12), 3);
    }
}
