use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

fn read_number(input: &str) -> Option<(u32, usize)> {
    if !input.chars().next().is_some_and(|c| c.is_numeric()) {
        return None;
    }
    let end_idx = input.find(|c: char| !c.is_numeric()).unwrap_or(input.len());
    input[..end_idx].parse().ok().map(|n| (n, end_idx))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut symbol_pos = HashSet::new();
    let mut number_table = Vec::new();
    input.lines().enumerate().for_each(|(line_idx, line)| {
        let mut start_idx = 0;
        while start_idx < line.len() {
            if let Some((number, len)) = read_number(&line[start_idx..]) {
                number_table.push((
                    number,
                    (start_idx..start_idx + len)
                        .map(|x| (x, line_idx))
                        .collect::<Vec<_>>(),
                ));
                start_idx += len;
            } else {
                if &line[start_idx..start_idx + 1] != "." {
                    for x_delta in -1..=1 {
                        for y_delta in -1..=1 {
                            let x = start_idx.checked_add_signed(x_delta);
                            let y = line_idx.checked_add_signed(y_delta);
                            if let Some(x) = x {
                                if let Some(y) = y {
                                    symbol_pos.insert((x, y));
                                }
                            }
                        }
                    }
                }
                start_idx += 1;
            }
        }
    });

    Some(
        number_table
            .iter_mut()
            .filter_map(|(number, positions)| {
                positions
                    .iter()
                    .any(|pos| symbol_pos.contains(pos))
                    .then_some(*number)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gear_pos = HashSet::new();
    let mut number_table: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    input.lines().enumerate().for_each(|(line_idx, line)| {
        let mut start_idx = 0;
        while start_idx < line.len() {
            if let Some((number, len)) = read_number(&line[start_idx..]) {
                for x_delta in -1..=len as isize {
                    for y_delta in -1..=1 {
                        let x = start_idx.checked_add_signed(x_delta);
                        let y = line_idx.checked_add_signed(y_delta);
                        if let Some(x) = x {
                            if let Some(y) = y {
                                if x >= start_idx && x < start_idx + len && y == line_idx {
                                    continue;
                                }
                                match number_table.get_mut(&(x, y)) {
                                    Some(v) => {
                                        v.push(number);
                                    }
                                    None => {
                                        number_table.insert((x, y), vec![number]);
                                    }
                                }
                            }
                        }
                    }
                }
                start_idx += len;
            } else {
                if &line[start_idx..start_idx + 1] == "*" {
                    gear_pos.insert((start_idx, line_idx));
                }
                start_idx += 1;
            }
        }
    });

    Some(
        gear_pos
            .iter()
            .filter_map(|pos| {
                number_table.get(pos).and_then(|v| {
                    if v.len() == 2 {
                        Some(v[0] * v[1])
                    } else {
                        None
                    }
                })
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
