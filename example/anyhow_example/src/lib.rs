use anyhow::{self, bail, Context};

use error::AppError;

pub fn parse_even(number: &str) -> anyhow::Result<i32> {
    let number: i32 = number.parse()?;

    if number % 2 != 0 {
        bail!(AppError::InvalidArgument("number is not even".to_string()));
    }
    Ok(number)
}

pub fn parse_even2(number: &str) -> anyhow::Result<i32> {
    let number: i32 = number
        .parse()
        .with_context(|| AppError::InvalidArgument(format!("failed to parse: {number}")))?;

    if number % 2 != 0 {
        bail!(AppError::InvalidArgument("number is not even".to_string()));
    }
    Ok(number)
}

/*
pub fn parse_even_bad(number: &str) -> Result<i32, AppError> {
    let number: i32 = number.parse()?;

    if number % 2 != 0 {
        return Err(AppError::InvalidArgument("number is not even".to_string()));
    }
    Ok(number)
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_even2() {
        let err = parse_even2("not integer").unwrap_err();
        println!("{err:?}");
    }
}
