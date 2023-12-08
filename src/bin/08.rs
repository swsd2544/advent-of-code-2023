use std::collections::HashMap;

advent_of_code::solution!(8);

type Directions = Vec<usize>;
type Nodes<'a> = HashMap<&'a str, [&'a str; 2]>;

fn parse_directions(input: &str) -> Directions {
    input
        .chars()
        .filter_map(|c| match c {
            'L' => Some(0),
            'R' => Some(1),
            _ => None,
        })
        .collect()
}

fn parse_nodes(input: &str) -> Nodes {
    input
        .lines()
        .filter_map(|line| {
            let (src, dest) = line.split_once(" = ")?;
            let (left_node, right_node) = dest
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();
            Some((src, [left_node, right_node]))
        })
        .collect()
}

fn parse_input(input: &str) -> Option<(Directions, Nodes)> {
    input
        .split_once("\n\n")
        .map(|(direction, nodes)| (parse_directions(direction), parse_nodes(nodes)))
}

fn find_total_steps_to_destination(
    start: &str,
    dest_f: fn(&str) -> bool,
    directions: &Directions,
    nodes: &Nodes,
) -> Option<u64> {
    let mut total_steps = 0;
    let mut current_node = start;
    while !dest_f(current_node) {
        let left_right_nodes = nodes.get(current_node)?;
        current_node = left_right_nodes[directions[total_steps % directions.len()]];
        total_steps += 1;
    }
    Some(total_steps as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (directions, nodes) = parse_input(input)?;
    find_total_steps_to_destination("AAA", |node: &str| node.eq("ZZZ"), &directions, &nodes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, nodes) = parse_input(input)?;
    let total_steps = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .filter_map(|node| {
            find_total_steps_to_destination(node, |node| node.ends_with('Z'), &directions, &nodes)
        })
        .fold(1, num::integer::lcm);
    Some(total_steps as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn another_test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
