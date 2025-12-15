use super::part1::parse_input;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, coords) = parse_input(input).expect("parse failed");

    todo!("day 01 - part 2");
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
        assert_eq!("", process(input)?);
        Ok(())
    }
}
