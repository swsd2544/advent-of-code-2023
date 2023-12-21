use std::collections::HashMap;

use itertools::iproduct;

advent_of_code::solution!(19);

type Rating = Vec<u64>; // X M A S
struct Branch<'a>(&'a str, Option<(usize, u8, u64)>);
type Workflow<'a> = Vec<Branch<'a>>;

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Rating>) {
    let (workflows, ratings) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|l| {
            let (name, branches) = l.split_once('{').unwrap();
            let branches = branches[..branches.len() - 1]
                .split(',')
                .map(|b| {
                    if let Some((cond, target)) = b.split_once(':') {
                        let c = cond.as_bytes()[0];
                        let r = b"xmas".iter().position(|b| *b == c).unwrap();
                        Branch(
                            target,
                            Some((r, cond.as_bytes()[1], cond[2..].parse().unwrap())),
                        )
                    } else {
                        Branch(b, None)
                    }
                })
                .collect();
            (name, branches)
        })
        .collect();
    let ratings = ratings
        .lines()
        .map(|l| {
            l[1..l.len() - 1]
                .split(',')
                .map(|s| s[2..].parse().unwrap())
                .collect()
        })
        .collect();
    (workflows, ratings)
}

fn process_workflow(
    workflows: &HashMap<&str, Workflow>,
    workflow_name: &str,
    mut possible: [(u64, u64); 4],
) -> u64 {
    match workflow_name {
        "A" => return possible.into_iter().map(|(l, h)| h - l).product(),
        "R" => return 0,
        _ => (),
    };
    let mut total = 0;
    let workflow = workflows.get(workflow_name).unwrap();
    for rule in workflow {
        match *rule {
            Branch(target, None) => return total + process_workflow(workflows, target, possible),
            Branch(target, Some((idx, op, limit))) => match (possible[idx], op) {
                ((_, u), b'<') if u <= limit => {
                    return total + process_workflow(workflows, target, possible)
                }
                ((l, _), b'>') if l > limit => {
                    return total + process_workflow(workflows, target, possible)
                }
                ((l, u), b'<') if l < limit => {
                    possible[idx] = (l, limit);
                    total += process_workflow(workflows, target, possible);
                    possible[idx] = (limit, u);
                }
                ((l, u), b'>') if u > limit => {
                    possible[idx] = (limit + 1, u);
                    total += process_workflow(workflows, target, possible);
                    possible[idx] = (l, limit + 1);
                }
                _ => {}
            },
        }
    }
    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, ratings) = parse_input(input);
    let total_ratings = ratings
        .into_iter()
        .filter(|rating| {
            process_workflow(
                &workflows,
                "in",
                [
                    (rating[0], rating[0] + 1),
                    (rating[1], rating[1] + 1),
                    (rating[2], rating[2] + 1),
                    (rating[3], rating[3] + 1),
                ],
            ) == 1
        })
        .map(|rating| rating.iter().sum::<u64>())
        .sum::<u64>();
    Some(total_ratings)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse_input(input);
    let total_ratings = process_workflow(
        &workflows,
        "in",
        [(1, 4001), (1, 4001), (1, 4001), (1, 4001)],
    );
    Some(total_ratings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
