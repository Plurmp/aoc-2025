use std::collections::HashMap;

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take},
    character::complete::{line_ending, space1},
    multi::separated_list1,
};
use pathfinding::prelude::count_paths;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, edges) = parse_input(input).expect("parse failed");

    let total_paths = count_paths(
        "you",
        |&device| edges.get(device).cloned().unwrap(),
        |&device| device == "out",
    );

    Ok(total_paths.to_string())
}

pub fn parse_input(input: &str) -> IResult<&str, HashMap<&str, Vec<&str>>> {
    let (input, devices) = separated_list1(line_ending, parse_device).parse(input)?;

    Ok((input, devices.into_iter().collect()))
}

fn parse_device(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, (device, _, connections)) = (
        take(3usize),
        tag(": "),
        separated_list1(space1, take(3usize)),
    )
        .parse(input)?;

    Ok((input, (device, connections)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
