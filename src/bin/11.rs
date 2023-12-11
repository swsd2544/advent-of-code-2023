use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut galaxy = Vec::new();
    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            if c == '#' {
                rows.insert(row_idx);
                cols.insert(col_idx);
                galaxy.push((row_idx, col_idx))
            }
        }
    }

    let max_row_idx = *rows.iter().max().unwrap_or(&0);
    let row_idx_offset: Vec<usize> = (0..=max_row_idx).fold(vec![], |mut acc, idx| {
        let prev_offset = acc.last().unwrap_or(&0);
        if rows.contains(&idx) {
            acc.push(*prev_offset);
        } else {
            acc.push(*prev_offset + 2 - 1);
        }
        acc
    });

    let max_col_idx = *cols.iter().max().unwrap_or(&0);
    let col_idx_offset: Vec<usize> = (0..=max_col_idx).fold(vec![], |mut acc, idx| {
        let prev_offset = acc.last().unwrap_or(&0);
        if cols.contains(&idx) {
            acc.push(*prev_offset);
        } else {
            acc.push(*prev_offset + 2 - 1);
        }
        acc
    });

    galaxy.iter_mut().for_each(|(row_idx, col_idx)| {
        *row_idx += row_idx_offset[*row_idx];
        *col_idx += col_idx_offset[*col_idx];
    });

    let sum_shortest_path = galaxy
        .iter()
        .enumerate()
        .flat_map(|(i, p)| galaxy[i + 1..].iter().map(move |p2| (p, p2)))
        .map(|(p, p2)| p.0.abs_diff(p2.0) + p.1.abs_diff(p2.1))
        .map(|v| v as u32)
        .sum();

    Some(sum_shortest_path)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut galaxy = Vec::new();
    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            if c == '#' {
                rows.insert(row_idx);
                cols.insert(col_idx);
                galaxy.push((row_idx, col_idx))
            }
        }
    }

    let max_row_idx = *rows.iter().max().unwrap_or(&0);
    let row_idx_offset: Vec<usize> = (0..=max_row_idx).fold(vec![], |mut acc, idx| {
        let prev_offset = acc.last().unwrap_or(&0);
        if rows.contains(&idx) {
            acc.push(*prev_offset);
        } else {
            acc.push(*prev_offset + 1000000 - 1);
        }
        acc
    });

    let max_col_idx = *cols.iter().max().unwrap_or(&0);
    let col_idx_offset: Vec<usize> = (0..=max_col_idx).fold(vec![], |mut acc, idx| {
        let prev_offset = acc.last().unwrap_or(&0);
        if cols.contains(&idx) {
            acc.push(*prev_offset);
        } else {
            acc.push(*prev_offset + 1000000 - 1);
        }
        acc
    });

    galaxy.iter_mut().for_each(|(row_idx, col_idx)| {
        *row_idx += row_idx_offset[*row_idx];
        *col_idx += col_idx_offset[*col_idx];
    });

    let sum_shortest_path = galaxy
        .iter()
        .enumerate()
        .flat_map(|(i, p)| galaxy[i + 1..].iter().map(move |p2| (p, p2)))
        .map(|(p, p2)| p.0.abs_diff(p2.0) + p.1.abs_diff(p2.1))
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
