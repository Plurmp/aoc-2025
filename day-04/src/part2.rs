use std::collections::HashSet;

use glam::{IVec2, ivec2};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut roll_locations: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(x, ch)| (ch == '@').then(|| IVec2::new(x as i32, y as i32)))
        })
        .collect();

    let width = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<String>()
        .len() as i32;
    let height = input.lines().collect::<Vec<_>>().len() as i32;
    let mut removed = 0usize;
    loop {
        let accessible_rolls: HashSet<_> = roll_locations
            .iter()
            .filter(|roll| check_adjacent(**roll, &roll_locations, width, height) < 4)
            .cloned()
            .collect();
        if accessible_rolls.is_empty() {
            break;
        }
        removed += accessible_rolls.len();
        roll_locations = &roll_locations - &accessible_rolls;
    }

    Ok(removed.to_string())
}

const ADJACENT: [IVec2; 8] = [
    ivec2(-1, -1),
    ivec2(0, -1),
    ivec2(1, -1),
    ivec2(-1, 0),
    ivec2(1, 0),
    ivec2(-1, 1),
    ivec2(0, 1),
    ivec2(1, 1),
];

fn check_adjacent(roll: IVec2, roll_locations: &HashSet<IVec2>, width: i32, height: i32) -> usize {
    ADJACENT
        .iter()
        .map(|adj| adj + roll)
        .filter(|adj| (0..width).contains(&adj.x) && (0..height).contains(&adj.y))
        .filter(|adj| roll_locations.contains(adj))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
