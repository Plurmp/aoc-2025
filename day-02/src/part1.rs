use nom::{
    IResult, Parser, bytes::complete::tag, character::complete, multi::separated_list1,
    sequence::separated_pair,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let ranges: Vec<_> = parse_ranges(input)
        .expect("parse failed")
        .1
        .into_iter()
        .map(|(start, end)| start..=end)
        .collect();
    dbg!(&ranges);

    let total: u64 = ranges
        .into_iter()
        .map(|range| range.filter(|n| is_invalid(*n)).sum::<u64>())
        .sum();

    Ok(total.to_string())
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(tag(","), parse_range).parse(input)
}

fn parse_range(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(complete::u64, tag("-"), complete::u64).parse(input)
}

fn is_invalid(n: u64) -> bool {
    let n_chars: Vec<_> = n.to_string().chars().collect();
    let n_len = n_chars.len();

    for div in (2..(n.isqrt() as usize)).step_by(2) {
        if n_len % div == 0 {
            let i = n_len / div;
            let copy = n_chars[..i].repeat(div);
            if copy == n_chars {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }

    #[test]
    fn test_is_invalid() -> miette::Result<()> {
        assert_eq!(true, is_invalid(1188511885));
        Ok(())
    }
}
