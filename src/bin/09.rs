advent_of_code::solution!(9);

fn next_number(sequence: &[i64]) -> Option<i64> {
    if sequence.iter().all(|&n| n == 0) {
        Some(0)
    } else {
        let differences: Vec<_> = sequence.windows(2).map(|w| w[1] - w[0]).collect();
        let diff = next_number(&differences)?;
        sequence.last().map(|n| n + diff)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let extrapolated_sum = input
        .lines()
        .filter_map(|line| {
            let sequence: Vec<_> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            next_number(&sequence)
        })
        .sum();
    Some(extrapolated_sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let extrapolated_sum = input
        .lines()
        .filter_map(|line| {
            let sequence: Vec<_> = line
                .split_whitespace()
                .rev()
                .map(|n| n.parse().unwrap())
                .collect();
            next_number(&sequence)
        })
        .sum();
    Some(extrapolated_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
