use std::collections::HashMap;

use itertools::Itertools;
use partitions::partition_vec;

use super::part1::parse_input;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, junctions) = parse_input(input).expect("parse failed");
    let sorted_connections: Vec<_> = junctions
        .iter()
        .tuple_combinations()
        .sorted_unstable_by_key(|(a, b)| a.distance_squared(**b))
        .collect();
    let junction_indices: HashMap<_, _> = junctions
        .iter()
        .enumerate()
        .map(|(i, junction)| (junction, i))
        .collect();
    let mut union_find = partition_vec![(); junctions.len()];
    let mut last_connection = sorted_connections[0];

    for connection in sorted_connections {
        if union_find.amount_of_sets() == 1 {
            break;
        }

        union_find.union(
            junction_indices[&connection.0],
            junction_indices[&connection.1],
        );
        last_connection = connection;
    }

    Ok((last_connection.0.x * last_connection.1.x).to_string())
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
425,690,689";
        assert_eq!("25272", process(input)?);
        Ok(())
    }
}
