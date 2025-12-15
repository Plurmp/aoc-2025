use pathfinding::prelude::count_paths;

use super::part1::parse_input;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, edges) = parse_input(input).expect("parse failed");

    let total_paths = count_paths(
        ("svr", false, false),
        |&(device, dac, fft)| {
            let dac = dac || device == "dac";
            let fft = fft || device == "fft";

            edges
                .get(device)
                .cloned()
                .unwrap_or(vec![])
                .into_iter()
                .map(move |connection| (connection, dac, fft))
        },
        |&(device, dac, fft)| device == "out" && dac && fft,
    );

    Ok(total_paths.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
