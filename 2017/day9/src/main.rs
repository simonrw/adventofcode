fn main() {}

pub fn group_score<T>(input: &mut T) -> u32
where
    T: Iterator<Item = char>,
{
    let mut current = input.next().unwrap();
    let mut depth = 0;
    let mut total = 0;

    loop {
        if let Some(next) = input.next() {

            match current {
                '{' => {
                    depth += 1;
                }
                '}' => {
                    depth -= 1;

                    total += depth;
                }
                _ => {}
            }

            current = next;
        } else {
            break;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "{}";
        let mut chars = input.chars();
        assert_eq!(group_score(&mut chars), 1);
    }

    #[test]
    fn test_second() {
        let input = "{{{}}}";
        let mut chars = input.chars();
        assert_eq!(group_score(&mut chars), 6);
    }

    #[test]
    fn test_third() {
        let input = "{{},{}}";
        let mut chars = input.chars();
        assert_eq!(group_score(&mut chars), 5);
    }

    #[test]
    fn test_fourth() {
        let input = "{{{},{},{{}}}}";
        let mut chars = input.chars();
        assert_eq!(group_score(&mut chars), 16);
    }

    #[test]
    fn test_fifth() {
        let input = "{<{},{},{{}}>}";
        let mut chars = input.chars();
        assert_eq!(group_score(&mut chars), 1);
    }
}
