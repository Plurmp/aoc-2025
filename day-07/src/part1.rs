use std::collections::HashSet;

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

    let splits = dfs(start_pos, &splitters, height, &mut HashSet::new());

    Ok(splits.to_string())
}

fn dfs(point: IVec2, splitters: &HashSet<IVec2>, height: i32, visited: &mut HashSet<IVec2>) -> i32 {
    if point.y >= height || visited.contains(&point) {
        return 0;
    }

    visited.insert(point);

    if splitters.contains(&point) {
        1 + dfs(point + IVec2::Y + IVec2::X, splitters, height, visited)
            + dfs(point + IVec2::Y + IVec2::NEG_X, splitters, height, visited)
    } else {
        dfs(point + IVec2::Y, splitters, height, visited)
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
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
