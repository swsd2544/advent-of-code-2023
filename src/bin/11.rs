use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(11);

fn parse_galaxy(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col_idx, _)| (row_idx, col_idx))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let galaxy = parse_galaxy(input);

    let contained_galaxy_rows: HashSet<_> = galaxy.iter().map(|&(row_idx, _)| row_idx).collect();
    let max_row_idx = contained_galaxy_rows
        .iter()
        .max()
        .copied()
        .unwrap_or_default();
    let row_idx_offset = (0..=max_row_idx).fold(Vec::<usize>::new(), |mut acc, idx| {
        let prev_offset = acc.last().unwrap_or(&0);
        if contained_galaxy_rows.contains(&idx) {
            acc.push(*prev_offset);
        } else {
            acc.push(*prev_offset + 2 - 1);
        }
        acc
    });

    let contained_galaxy_cols: HashSet<_> = galaxy.iter().map(|&(_, col_idx)| col_idx).collect();
    let max_col_idx = contained_galaxy_cols
        .iter()
        .max()
        .copied()
        .unwrap_or_default();
    let col_idx_offset = (0..=max_col_idx).fold(Vec::<usize>::new(), |mut acc, idx| {
        let prev_offset = acc.last().unwrap_or(&0);
        if contained_galaxy_cols.contains(&idx) {
            acc.push(*prev_offset);
        } else {
            acc.push(*prev_offset + 2 - 1);
        }
        acc
    });

    println!("{:?} {:?}", row_idx_offset, col_idx_offset);

    let sum_shortest_path = galaxy
        .iter()
        .map(|&(row_idx, col_idx)| {
            (
                row_idx + row_idx_offset[row_idx],
                col_idx + col_idx_offset[col_idx],
            )
        })
        .combinations(2)
        .map(|p| p[0].0.abs_diff(p[1].0) + p[0].1.abs_diff(p[1].1))
        .map(|v| v as u64)
        .sum();

    Some(sum_shortest_path)
}

pub fn part_two(input: &str) -> Option<u64> {
    let galaxy = parse_galaxy(input);

    let contained_galaxy_rows: HashSet<_> = galaxy.iter().map(|&(row_idx, _)| row_idx).collect();
    let max_row_idx = contained_galaxy_rows
        .iter()
        .max()
        .copied()
        .unwrap_or_default();
    let row_idx_offset = (0..=max_row_idx).fold(Vec::<usize>::new(), |mut acc, idx| {
        let prev_offset = acc.last().unwrap_or(&0);
        if contained_galaxy_rows.contains(&idx) {
            acc.push(*prev_offset);
        } else {
            acc.push(*prev_offset + 1000000 - 1);
        }
        acc
    });

    let contained_galaxy_cols: HashSet<_> = galaxy.iter().map(|&(_, col_idx)| col_idx).collect();
    let max_col_idx = contained_galaxy_cols
        .iter()
        .max()
        .copied()
        .unwrap_or_default();
    let col_idx_offset = (0..=max_col_idx).fold(Vec::<usize>::new(), |mut acc, idx| {
        let prev_offset = acc.last().unwrap_or(&0);
        if contained_galaxy_cols.contains(&idx) {
            acc.push(*prev_offset);
        } else {
            acc.push(*prev_offset + 1000000 - 1);
        }
        acc
    });

    let sum_shortest_path = galaxy
        .iter()
        .map(|&(row_idx, col_idx)| {
            (
                row_idx + row_idx_offset[row_idx],
                col_idx + col_idx_offset[col_idx],
            )
        })
        .combinations(2)
        .map(|p| p[0].0.abs_diff(p[1].0) + p[0].1.abs_diff(p[1].1))
        .map(|v| v as u64)
        .sum();

    Some(sum_shortest_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
