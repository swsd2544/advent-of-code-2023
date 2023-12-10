advent_of_code::solution!(10);

const UP_MASK: u8 = 0b1000;
const DOWN_MASK: u8 = 0b0100;
const LEFT_MASK: u8 = 0b0010;
const RIGHT_MASK: u8 = 0b0001;

fn encode_pipe_directions(c: char) -> u8 {
    match c {
        '|' => UP_MASK | DOWN_MASK,
        '-' => LEFT_MASK | RIGHT_MASK,
        'L' => UP_MASK | RIGHT_MASK,
        'J' => UP_MASK | LEFT_MASK,
        '7' => DOWN_MASK | LEFT_MASK,
        'F' => DOWN_MASK | RIGHT_MASK,
        'S' => UP_MASK | DOWN_MASK | LEFT_MASK | RIGHT_MASK,
        _ => 0,
    }
}

fn parse_grid(input: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(encode_pipe_directions).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let starting_position = grid
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .position(|&x| x == UP_MASK | DOWN_MASK | LEFT_MASK | RIGHT_MASK)
                .map(|col_idx| (row_idx, col_idx))
        })
        .unwrap();
    if starting_position.0 > 0
        && (grid[starting_position.0 - 1][starting_position.1] & DOWN_MASK) == 0
    {
        grid[starting_position.0][starting_position.1] &= !UP_MASK;
    }
    if starting_position.0 < grid.len() - 1
        && (grid[starting_position.0 + 1][starting_position.1] & UP_MASK) == 0
    {
        grid[starting_position.0][starting_position.1] &= !DOWN_MASK;
    }
    if starting_position.1 > 0
        && (grid[starting_position.0][starting_position.1 - 1] & RIGHT_MASK) == 0
    {
        grid[starting_position.0][starting_position.1] &= !LEFT_MASK;
    }
    if starting_position.0 < grid[0].len() - 1
        && (grid[starting_position.0][starting_position.1 + 1] & LEFT_MASK) == 0
    {
        grid[starting_position.0][starting_position.1] &= !RIGHT_MASK;
    }
    (grid, starting_position)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, starting_position) = parse_grid(input);

    let mut loop_cells = Vec::new();
    let mut prev_dir = 0;
    let mut curr_pos = starting_position;
    loop {
        loop_cells.push(curr_pos);
        let next_pos = if (grid[curr_pos.0][curr_pos.1] & DOWN_MASK) != 0 && prev_dir != DOWN_MASK {
            prev_dir = UP_MASK;
            (curr_pos.0 + 1, curr_pos.1)
        } else if (grid[curr_pos.0][curr_pos.1] & RIGHT_MASK) != 0 && prev_dir != RIGHT_MASK {
            prev_dir = LEFT_MASK;
            (curr_pos.0, curr_pos.1 + 1)
        } else if (grid[curr_pos.0][curr_pos.1] & UP_MASK) != 0 && prev_dir != UP_MASK {
            prev_dir = DOWN_MASK;
            (curr_pos.0 - 1, curr_pos.1)
        } else if (grid[curr_pos.0][curr_pos.1] & LEFT_MASK) != 0 && prev_dir != LEFT_MASK {
            prev_dir = RIGHT_MASK;
            (curr_pos.0, curr_pos.1 - 1)
        } else {
            panic!("No direction available");
        };

        if next_pos == starting_position {
            break;
        } else {
            curr_pos = next_pos;
        }
    }

    Some(loop_cells.len().div_ceil(2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, starting_position) = parse_grid(input);

    let mut loop_cells = Vec::new();
    let mut prev_dir = 0;
    let mut curr_pos = starting_position;
    loop {
        loop_cells.push(curr_pos);
        let next_pos = if (grid[curr_pos.0][curr_pos.1] & DOWN_MASK) != 0 && prev_dir != DOWN_MASK {
            prev_dir = UP_MASK;
            (curr_pos.0 + 1, curr_pos.1)
        } else if (grid[curr_pos.0][curr_pos.1] & RIGHT_MASK) != 0 && prev_dir != RIGHT_MASK {
            prev_dir = LEFT_MASK;
            (curr_pos.0, curr_pos.1 + 1)
        } else if (grid[curr_pos.0][curr_pos.1] & UP_MASK) != 0 && prev_dir != UP_MASK {
            prev_dir = DOWN_MASK;
            (curr_pos.0 - 1, curr_pos.1)
        } else if (grid[curr_pos.0][curr_pos.1] & LEFT_MASK) != 0 && prev_dir != LEFT_MASK {
            prev_dir = RIGHT_MASK;
            (curr_pos.0, curr_pos.1 - 1)
        } else {
            panic!("No direction available");
        };

        if next_pos == starting_position {
            break;
        } else {
            curr_pos = next_pos;
        }
    }

    let vertices: Vec<_> = loop_cells
        .iter()
        .filter(|(row_idx, col_idx)| {
            grid[*row_idx][*col_idx] != (UP_MASK | DOWN_MASK)
                && grid[*row_idx][*col_idx] != (LEFT_MASK | RIGHT_MASK)
        })
        .map(|&(row_idx, col_idx)| (col_idx, grid.len() - row_idx))
        .collect();
    let mut two_area = 0;
    for i in 0..vertices.len() {
        let next_i = (i + 1) % vertices.len();
        two_area += (vertices[i].0 * vertices[next_i].1) as isize
            - (vertices[i].1 * vertices[next_i].0) as isize;
    }
    let inner_cells_count = (two_area.abs() - loop_cells.len() as isize) / 2 + 1;
    Some(inner_cells_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(10));
    }
}
