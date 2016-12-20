use std::error::Error;
type Result<T> = ::std::result::Result<T, Box<Error>>;

pub fn puzzle(input: &[u8]) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
    }
}
