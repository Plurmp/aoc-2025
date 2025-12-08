use std::collections::HashSet;

use cached::UnboundCache;
use cached::proc_macro::cached;
use glam::{IVec2, ivec2};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let start_pos = ivec2(
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .position(|ch| ch == 'S')
            .unwrap() as i32,
        0,
    );
    let splitters: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .flat_map(move |(x, ch)| (ch == '^').then(|| ivec2(x as i32, y as i32)))
        })
        .collect();
    let height = input.lines().count() as i32;

    let timelines = dfs(start_pos, &splitters, height) + 1;

    Ok(timelines.to_string())
}

#[cached(
    ty = "UnboundCache<IVec2, u64>",
    create = "{ UnboundCache::new() }",
    convert = "{ point }"
)]
fn dfs(point: IVec2, splitters: &HashSet<IVec2>, height: i32) -> u64 {
    if point.y >= height {
        return 0;
    }

    if splitters.contains(&point) {
        1 + dfs(point + IVec2::Y + IVec2::X, splitters, height)
            + dfs(point + IVec2::Y + IVec2::NEG_X, splitters, height)
    } else {
        dfs(point + IVec2::Y, splitters, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("40", process(input)?);
        Ok(())
    }
}
