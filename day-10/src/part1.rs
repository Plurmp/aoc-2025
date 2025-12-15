use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list0, separated_list1},
};

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u16>,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = parse_input(input).expect("parse failed");

    let total = machines
        .into_iter()
        .map(|machine| {
            for len in 1..machine.buttons.len() {
                for buttons in machine.buttons.iter().combinations(len) {
                    let mut test_buttons = vec![false; machine.lights.len()];
                    for button in buttons {
                        for &i in button {
                            test_buttons[i] = !test_buttons[i];
                        }
                    }
                    if test_buttons == machine.lights {
                        return len;
                    }
                }
            }

            panic!("did not find a button combination");
        })
        .sum::<usize>();

    Ok(total.to_string())
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(line_ending, parse_machine).parse(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (lights, _, buttons, _, joltage)) =
        (parse_lights, space1, parse_buttons, space1, parse_joltage).parse(input)?;

    Ok((
        input,
        Machine {
            lights,
            buttons,
            joltage,
        },
    ))
}

fn parse_lights(input: &str) -> IResult<&str, Vec<bool>> {
    let (input, _) = tag("[")(input)?;
    let (input, lights) = many1(alt((tag("."), tag("#")))).parse(input)?;
    let (input, _) = tag("]")(input)?;

    let lights = lights
        .into_iter()
        .map(|s| match s {
            "." => false,
            "#" => true,
            _ => unreachable!(),
        })
        .collect();

    Ok((input, lights))
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list0(space1, parse_button).parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("(")(input)?;
    let (input, button) = separated_list1(tag(","), complete::usize).parse(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, button))
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<u16>> {
    let (input, _) = tag("{")(input)?;
    let (input, joltage) = separated_list1(tag(","), complete::u16).parse(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, joltage))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
