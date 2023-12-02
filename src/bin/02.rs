use std::{collections::HashMap, ops::Not};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_balls = HashMap::new();
    total_balls.insert("red", 12);
    total_balls.insert("green", 13);
    total_balls.insert("blue", 14);

    Some(
        input
            .lines()
            .filter_map(|line| {
                let (id, rounds) = line
                    .split_once(':')
                    .map(|(id, content)| {
                        (
                            id.strip_prefix("Game ")
                                .map(|id| id.parse::<u32>().unwrap())
                                .unwrap(),
                            content,
                        )
                    })
                    .unwrap();
                rounds
                    .split(';')
                    .any(|round| {
                        round.split(", ").any(|s| {
                            let (n, color) = s
                                .trim()
                                .split_once(' ')
                                .map(|(n, color)| (n.parse::<i32>().unwrap(), color))
                                .unwrap();
                            total_balls.get(&color).is_some_and(|max_n| n > *max_n)
                        })
                    })
                    .not()
                    .then_some(id)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, rounds) = line
                    .split_once(':')
                    .map(|(id, content)| {
                        (
                            id.strip_prefix("Game ")
                                .map(|id| id.parse::<u32>().unwrap())
                                .unwrap(),
                            content,
                        )
                    })
                    .unwrap();

                let mut total_balls: HashMap<&str, u32> = HashMap::new();

                rounds.split(';').for_each(|round| {
                    round.split(", ").for_each(|s| {
                        let (n, color) = s
                            .trim()
                            .split_once(' ')
                            .map(|(n, color)| (n.parse::<u32>().unwrap(), color))
                            .unwrap();
                        if n > total_balls.get(&color).cloned().unwrap_or_default() {
                            total_balls.insert(color, n);
                        }
                    })
                });

                total_balls.get("red").cloned().unwrap_or_default()
                    * total_balls.get("green").cloned().unwrap_or_default()
                    * total_balls.get("blue").cloned().unwrap_or_default()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
