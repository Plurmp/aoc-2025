#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut zero_count = 0;
    let mut dial_pos = 50;
    for line in input.lines() {
        let (prefix, n) = line.split_at(1);
        let n = n.parse::<i32>().unwrap();
        
        let next = match prefix {
            "L" => dial_pos - n,
            "R" => dial_pos + n,
            _ => unreachable!(),
        };
        let (div, rem) = (next / 100, next.rem_euclid(100));
        zero_count += div.abs();
        if dial_pos != 0 && next <= 0 {
            zero_count += 1;
        }
        dial_pos = rem;
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
