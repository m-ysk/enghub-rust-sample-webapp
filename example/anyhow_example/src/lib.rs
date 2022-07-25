use anyhow::{self, bail};

use error::AppError;

pub fn parse_even(number: String) -> anyhow::Result<i32> {
    let number: i32 = number.parse()?;

    if number % 2 != 0 {
        bail!(AppError::InvalidArgument("number is not even".to_string()));
    }
    Ok(number)
}
