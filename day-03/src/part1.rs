use std::cmp::Reverse;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let banks: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let total: u32 = banks
        .into_iter()
        .map(|bank| {
            let (first_digit, Reverse(first_digit_pos)) = bank[..(bank.len() - 1)]
                .iter()
                .enumerate()
                .map(|(i, &n)| (n, Reverse(i)))
                .max()
                .unwrap();
            let second_digit = *bank[(first_digit_pos + 1)..].iter().max().unwrap();
            first_digit * 10 + second_digit
        })
        .sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!("357", process(input)?);
        Ok(())
    }
}
