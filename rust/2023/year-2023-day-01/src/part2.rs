use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let res = input
        .lines()
        .map(|line| {
            let first = (0..line.len()).find_map(|i| find_in_slice(&line[i..]));
            let last = (0..line.len())
                .rev()
                .find_map(|i| find_in_slice(&line[i..]));

            match (first, last) {
                (Some(a), Some(b)) => a * 10 + b,
                _ => 0,
            }
        })
        .sum::<u32>()
        .to_string();
    Ok(res)
}

const STR_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_in_slice(slice: &str) -> Option<u32> {
    STR_NUMBERS
        .iter()
        .position(|str_digit| slice.starts_with(str_digit))
        .map(|i| i as u32 + 1)
        .or_else(|| slice.chars().next().and_then(|c| c.to_digit(10)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
