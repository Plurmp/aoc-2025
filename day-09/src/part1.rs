use glam::{I64Vec2, i64vec2};
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, coords) = parse_input(input).expect("parse failed");

    let largest_rect = coords
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| ((a - b).abs() + I64Vec2::X + I64Vec2::Y).element_product())
        .max()
        .unwrap();

    Ok(largest_rect.to_string())
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<I64Vec2>> {
    separated_list1(line_ending, parse_coord).parse(input)
}

fn parse_coord(input: &str) -> IResult<&str, I64Vec2> {
    let (input, (x, y)) = separated_pair(complete::i64, tag(","), complete::i64).parse(input)?;

    Ok((input, i64vec2(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!("50", process(input)?);
        Ok(())
    }
}
