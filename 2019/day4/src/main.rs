fn digits(n: i32) -> Vec<i32> {
    fn x_inner(n: i32, xs: &mut Vec<i32>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}
struct Password {
    pub min: i32,
    pub max: i32,
}

impl Password {
    fn new(min: i32, max: i32) -> Self {
        Password { min, max }
    }

    fn is_valid(&self, n: i32) -> bool {
        if n < self.min {
            return false;
        }

        if n > self.max {
            return false;
        }

        // check if a pair of numbers exists
        let d = digits(n);

        let mut found_double = false;
        for i in 0..5 {
            if d[i] == d[i + 1] {
                // Check that we have an even number of this value
                let n_same = d.iter().filter(|v| **v == d[i]).count();
                assert!(n_same >= 2);
                if n_same % 2 != 0 {
                    return false;
                }

                found_double = true;
            }
        }

        if !found_double {
            return false;
        }

        // Check that all numbers are ascending
        let mut current = d[0];
        for i in 1..6 {
            if d[i] < current {
                return false;
            }
            current = d[i];
        }

        true
    }
}

fn main() {
    let password = Password::new(153517, 630395);

    let count = (password.min..=password.max)
        .filter(|v| password.is_valid(*v))
        .count();

    println!("Found {} valid passwords", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_1() {
        let password = Password::new(100000, 999999);
        assert!(password.is_valid(111111));
    }

    #[test]
    fn test_validation_2() {
        let password = Password::new(100000, 999999);
        assert!(!password.is_valid(223450));
    }

    #[test]
    fn test_validation_3() {
        let password = Password::new(100000, 999999);
        assert!(!password.is_valid(123789));
    }

    #[test]
    fn test_digits() {
        assert_eq!(digits(123456), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_validation_4() {
        let password = Password::new(100000, 999999);
        assert!(password.is_valid(112233));
    }

    #[test]
    fn test_validation_5() {
        let password = Password::new(100000, 999999);
        assert!(!password.is_valid(123444));
    }

    #[test]
    fn test_validation_6() {
        let password = Password::new(100000, 999999);
        assert!(password.is_valid(111122));
    }
}
