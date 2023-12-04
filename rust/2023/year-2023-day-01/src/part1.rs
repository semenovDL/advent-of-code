use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let res = input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10));
            let last = line.chars().rev().find_map(|c| c.to_digit(10));

            match (first, last) {
                (Some(a), Some(b)) => a * 10 + b,
                _ => 0,
            }
        })
        .sum::<u32>()
        .to_string();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
