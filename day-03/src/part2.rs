use std::cmp::Reverse;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let banks: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let total: u64 = banks.into_iter().map(|bank| largest_joltage(&bank)).sum();

    Ok(total.to_string())
}

fn largest_joltage(bank: &[u32]) -> u64 {
    let mut ans = 0u64;
    let mut start_pos = 0usize;

    let bank: Vec<_> = bank.iter().enumerate().collect();
    for i in 1..=12 {
        // dbg!(ans, start_pos, &bank[start_pos..(bank.len() - 12 + i)]);
        let (largest_digit_pos, largest_digit) = bank[(start_pos)..(bank.len() - 12 + i)]
            .iter()
            .max_by_key(|(i, n)| (n, Reverse(i)))
            .cloned()
            .unwrap();

        ans = ans * 10 + *largest_digit as u64;
        start_pos = largest_digit_pos + 1;
    }

    ans
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
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }

    #[test]
    fn test_line_1() -> miette::Result<()> {
        let input = "987654321111111";
        assert_eq!(
            987654321111,
            largest_joltage(
                &input
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            )
        );
        Ok(())
    }

    #[test]
    fn test_line_2() -> miette::Result<()> {
        let input = "811111111111119";
        assert_eq!(
            811111111119,
            largest_joltage(
                &input
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            )
        );
        Ok(())
    }

    #[test]
    fn test_line_3() -> miette::Result<()> {
        let input = "234234234234278";
        assert_eq!(
            434234234278,
            largest_joltage(
                &input
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            )
        );
        Ok(())
    }

    #[test]
    fn test_line_4() -> miette::Result<()> {
        let input = "818181911112111";
        assert_eq!(
            888911112111,
            largest_joltage(
                &input
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            )
        );
        Ok(())
    }
}
