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
    min: i32,
    max: i32,
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

        let d = digits(n);

        let mut valid = false;
        for i in 0..5 {
            if d[i] == d[i + 1] {
                valid = true;
                break;
            }
        }

        if valid {
            return valid;
        }

        return false;
    }
}

fn main() {
    println!("Hello, world!");
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
}
