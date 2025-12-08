use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, space1},
    multi::separated_list1,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (lines, ops)) = parse_input(input).expect("parse failed");

    let n = lines[0].len();
    let mut total = 0u64;

    for i in 0..n {
        total += match ops[i] {
            Op::Add => lines.iter().map(|line| line[i]).sum::<u64>(),
            Op::Mult => lines.iter().map(|line| line[i]).product::<u64>(),
        };
    }

    Ok(total.to_string())
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mult,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Op>)> {
    let (input, num_lines) = parse_num_lines(input)?;
    let (input, _) = multispace1(input)?;
    let (input, ops) = parse_ops(input)?;

    Ok((input, (num_lines, ops)))
}

fn parse_num_lines(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(multispace1, parse_num_line).parse(input)
}

fn parse_num_line(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64).parse(input)
}

fn parse_ops(input: &str) -> IResult<&str, Vec<Op>> {
    let (input, ops) = separated_list1(space1, alt((tag("+"), tag("*")))).parse(input)?;

    let ops = ops
        .into_iter()
        .map(|op| match op {
            "+" => Op::Add,
            "*" => Op::Mult,
            _ => unreachable!(),
        })
        .collect();

    Ok((input, ops))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!("4277556", process(input)?);
        Ok(())
    }
}
