#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut zero_count = 0;
    let mut dial_pos = 50;
    for line in input.lines() {
        let (prefix, n) = line.split_at(1);
        match prefix {
            "L" => {
                dial_pos = (dial_pos
                    - n.parse::<i32>().unwrap())
                .rem_euclid(100)
            }
            "R" => {
                dial_pos = (dial_pos
                    + n.parse::<i32>().unwrap())
                .rem_euclid(100)
            }
            _ => unreachable!(),
        }
        if dial_pos == 0 {
            zero_count += 1;
        }
    }
    Ok(zero_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
