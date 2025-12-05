use std::ops::RangeInclusive;

use nom::{IResult, Parser, bytes::complete::tag, character::complete::{self, line_ending}, multi::separated_list1, sequence::separated_pair};
use range_set_blaze::RangeSetBlaze;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (ranges, ids)) = parse_input(input).expect("parse failed");

    let range_set = RangeSetBlaze::from_iter(ranges.into_iter());

    let fresh = ids
        .into_iter()
        .filter(|id| range_set.contains(*id))
        .count();

    Ok(fresh.to_string())
}

fn parse_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (input, ranges) = parse_ranges(input)?;
    let (input, _) = (line_ending, line_ending).parse(input)?;
    let (input, ids) = separated_list1(line_ending, complete::u64).parse(input)?;

    Ok((input, (ranges, ids)))
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(line_ending, parse_range).parse(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (start, end)) = separated_pair(complete::u64, tag("-"), complete::u64).parse(input)?;

    Ok((input, start..=end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}