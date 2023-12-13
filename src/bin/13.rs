advent_of_code::solution!(13);

#[derive(PartialEq, Eq)]
enum Position {
    Ash,
    Rock,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("unknown position value"),
        }
    }
}

enum MirrorLine {
    NotFound,
    Vertical { total_left_cols: usize },
    Horizontal { total_top_rows: usize },
}

fn find_mirror_line(pattern: &Vec<Vec<Position>>, smudges_count: usize) -> MirrorLine {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();

    for total_left_cols in 1..num_cols {
        let left_cols = (0..total_left_cols).rev();
        let right_cols = total_left_cols..num_cols;
        let mut total_diff = 0;
        for (left_idx, right_idx) in left_cols.zip(right_cols) {
            (0..num_rows).for_each(|row_idx| {
                if pattern[row_idx][left_idx] != pattern[row_idx][right_idx] {
                    total_diff += 1;
                }
            });
        }
        if total_diff == smudges_count {
            return MirrorLine::Vertical { total_left_cols };
        }
    }

    for total_top_rows in 1..num_rows {
        let top_rows = (0..total_top_rows).rev();
        let bottom_rows = total_top_rows..num_rows;
        let mut total_diff = 0;
        for (top_idx, bottom_idx) in top_rows.zip(bottom_rows) {
            (0..num_cols).for_each(|col_idx| {
                if pattern[top_idx][col_idx] != pattern[bottom_idx][col_idx] {
                    total_diff += 1;
                }
            });
        }
        if total_diff == smudges_count {
            return MirrorLine::Horizontal { total_top_rows };
        }
    }

    MirrorLine::NotFound
}

pub fn part_one(input: &str) -> Option<u32> {
    let summary = input
        .split("\n\n")
        .map(|input| {
            let pattern: Vec<Vec<_>> = input
                .lines()
                .map(|line| line.chars().map(Position::from).collect())
                .collect();
            match find_mirror_line(&pattern, 0) {
                MirrorLine::NotFound => 0,
                MirrorLine::Vertical { total_left_cols } => total_left_cols,
                MirrorLine::Horizontal { total_top_rows } => 100 * total_top_rows,
            }
        })
        .sum::<usize>();
    Some(summary as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let summary = input
        .split("\n\n")
        .map(|input| {
            let pattern: Vec<Vec<_>> = input
                .lines()
                .map(|line| line.chars().map(Position::from).collect())
                .collect();
            match find_mirror_line(&pattern, 1) {
                MirrorLine::NotFound => 0,
                MirrorLine::Vertical { total_left_cols } => total_left_cols,
                MirrorLine::Horizontal { total_top_rows } => 100 * total_top_rows,
            }
        })
        .sum::<usize>();
    Some(summary as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
