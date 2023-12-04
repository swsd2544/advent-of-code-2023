use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let base_number: u32 = 2;
    Some(
        input
            .lines()
            .filter_map(|line| {
                let colon_idx = line.find(':')?;
                let (winning_numbers, card_numbers) =
                    &line[colon_idx + 1..].split_once('|').map(|(left, right)| {
                        (
                            left.split_whitespace()
                                .filter_map(|n| n.parse::<u32>().ok())
                                .collect::<HashSet<_>>(),
                            right
                                .split_whitespace()
                                .filter_map(|n| n.parse::<u32>().ok())
                                .collect::<HashSet<_>>(),
                        )
                    })?;
                winning_numbers
                    .intersection(card_numbers)
                    .count()
                    .checked_add_signed(-1)
            })
            .map(|n| base_number.pow(n as u32))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let winning_numbers_table = input
        .lines()
        .filter_map(|line| {
            let colon_idx = line.find(':')?;
            let (winning_numbers, card_numbers) =
                &line[colon_idx + 1..].split_once('|').map(|(left, right)| {
                    (
                        left.split_whitespace()
                            .filter_map(|n| n.parse::<u32>().ok())
                            .collect::<HashSet<_>>(),
                        right
                            .split_whitespace()
                            .filter_map(|n| n.parse::<u32>().ok())
                            .collect::<HashSet<_>>(),
                    )
                })?;
            Some(winning_numbers.intersection(card_numbers).count())
        })
        .collect::<Vec<_>>();

    let mut scratchcards_count = vec![1; winning_numbers_table.len()];
    winning_numbers_table
        .iter()
        .enumerate()
        .for_each(|(idx, &count)| {
            (1..=count).for_each(|idx_delta| {
                scratchcards_count[idx + idx_delta] += scratchcards_count[idx];
            });
        });

    Some(scratchcards_count.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
