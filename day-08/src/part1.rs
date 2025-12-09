use std::collections::HashMap;

use glam::{I64Vec3, i64vec3};
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};
use partitions::partition_vec;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    solve(input, 1000)
}

fn solve(input: &str, k: usize) -> miette::Result<String> {
    let (_, junctions) = parse_input(input).expect("parse failed");
    let junction_indices: HashMap<_, _> = junctions
        .iter()
        .enumerate()
        .map(|(i, junction)| (junction, i))
        .collect();
    let shortest_connections = k_shortest_connections(k, &junctions);
    let mut union_find = partition_vec![(); junctions.len()];
    for connection in shortest_connections {
        union_find.union(
            junction_indices[&connection.0],
            junction_indices[&connection.1],
        );
    }

    let ans: usize = union_find
        .all_sets()
        .map(|set| set.count())
        .k_largest(3)
        .product();

    Ok(ans.to_string())
}

pub fn k_shortest_connections(k: usize, junctions: &[I64Vec3]) -> Vec<(I64Vec3, I64Vec3)> {
    junctions
        .iter()
        .tuple_combinations()
        .k_smallest_by_key(k, |(a, b)| a.distance_squared(**b))
        .map(|(a, b)| (*a, *b))
        .collect()
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<I64Vec3>> {
    separated_list1(line_ending, parse_junction).parse(input)
}

fn parse_junction(input: &str) -> IResult<&str, I64Vec3> {
    let (input, (x, _, y, _, z)) = (
        complete::i64,
        tag(","),
        complete::i64,
        tag(","),
        complete::i64,
    )
        .parse(input)?;

    Ok((input, i64vec3(x, y, z)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        assert_eq!("40", solve(input, 10)?);
        Ok(())
    }
}
