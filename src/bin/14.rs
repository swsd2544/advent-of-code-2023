use std::collections::HashMap;

advent_of_code::solution!(14);

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
enum Position {
    RoundedRock,
    CubeRock,
    Space,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::RoundedRock,
            '#' => Self::CubeRock,
            '.' => Self::Space,
            _ => panic!("unknown character"),
        }
    }
}

impl Into<char> for Position {
    fn into(self) -> char {
        match self {
            Self::Space => '.',
            Self::CubeRock => '#',
            Self::RoundedRock => 'O',
        }
    }
}

fn tilt_dish_northside(dish: &mut Vec<Vec<Position>>) {
    let num_rows = dish.len();
    let num_cols = dish[0].len();

    for col_idx in 0..num_cols {
        let mut rounded_rocks = 0;
        for row_idx in (0..num_rows).rev() {
            if let Position::CubeRock = dish[row_idx][col_idx] {
                for i in 1.. {
                    if rounded_rocks == 0 {
                        break;
                    }
                    if let Position::Space = dish[row_idx + i][col_idx] {
                        dish[row_idx + i][col_idx] = Position::RoundedRock;
                        rounded_rocks -= 1;
                    }
                }
            } else if let Position::RoundedRock = dish[row_idx][col_idx] {
                dish[row_idx][col_idx] = Position::Space;
                rounded_rocks += 1;
            }
        }
        for i in 0..rounded_rocks {
            dish[i][col_idx] = Position::RoundedRock;
        }
    }
}

fn tilt_dish_southside(dish: &mut Vec<Vec<Position>>) {
    let num_rows = dish.len();
    let num_cols = dish[0].len();

    for col_idx in 0..num_cols {
        let mut rounded_rocks = 0;
        for row_idx in 0..num_rows {
            if let Position::CubeRock = dish[row_idx][col_idx] {
                for i in 1.. {
                    if rounded_rocks == 0 {
                        break;
                    }
                    if let Position::Space = dish[row_idx - i][col_idx] {
                        dish[row_idx - i][col_idx] = Position::RoundedRock;
                        rounded_rocks -= 1;
                    }
                }
            } else if let Position::RoundedRock = dish[row_idx][col_idx] {
                dish[row_idx][col_idx] = Position::Space;
                rounded_rocks += 1;
            }
        }
        for i in 0..rounded_rocks {
            dish[num_rows - i - 1][col_idx] = Position::RoundedRock;
        }
    }
}

fn tilt_dish_westside(dish: &mut Vec<Vec<Position>>) {
    let num_rows = dish.len();
    let num_cols = dish[0].len();

    for row_idx in 0..num_rows {
        let mut rounded_rocks = 0;
        for col_idx in (0..num_cols).rev() {
            if let Position::CubeRock = dish[row_idx][col_idx] {
                for i in 1.. {
                    if rounded_rocks == 0 {
                        break;
                    }
                    if let Position::Space = dish[row_idx][col_idx + i] {
                        dish[row_idx][col_idx + i] = Position::RoundedRock;
                        rounded_rocks -= 1;
                    }
                }
            } else if let Position::RoundedRock = dish[row_idx][col_idx] {
                dish[row_idx][col_idx] = Position::Space;
                rounded_rocks += 1;
            }
        }
        for i in 0..rounded_rocks {
            dish[row_idx][i] = Position::RoundedRock;
        }
    }
}

fn tilt_dish_eastside(dish: &mut Vec<Vec<Position>>) {
    let num_rows = dish.len();
    let num_cols = dish[0].len();

    for row_idx in 0..num_rows {
        let mut rounded_rocks = 0;
        for col_idx in 0..num_cols {
            if let Position::CubeRock = dish[row_idx][col_idx] {
                for i in 1.. {
                    if rounded_rocks == 0 {
                        break;
                    }
                    if let Position::Space = dish[row_idx][col_idx - i] {
                        dish[row_idx][col_idx - i] = Position::RoundedRock;
                        rounded_rocks -= 1;
                    }
                }
            } else if let Position::RoundedRock = dish[row_idx][col_idx] {
                dish[row_idx][col_idx] = Position::Space;
                rounded_rocks += 1;
            }
        }
        for i in 0..rounded_rocks {
            dish[row_idx][num_cols - i - 1] = Position::RoundedRock;
        }
    }
}

fn rotate_dish(dish: &mut Vec<Vec<Position>>) {
    tilt_dish_northside(dish);
    tilt_dish_westside(dish);
    tilt_dish_southside(dish);
    tilt_dish_eastside(dish);
}

fn calculate_load_northside(dish: &Vec<Vec<Position>>) -> u32 {
    let num_rows = dish.len();
    let num_cols = dish[0].len();

    let mut load = 0;

    for col_idx in 0..num_cols {
        for row_idx in 0..num_rows {
            if let Position::RoundedRock = dish[row_idx][col_idx] {
                load += num_rows - row_idx;
            }
        }
    }

    load as u32
}

fn parse_dish(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .map(|line| line.chars().map(Position::from).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut dish = parse_dish(input);
    tilt_dish_northside(&mut dish);
    Some(calculate_load_northside(&dish))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut dish = parse_dish(input);
    let mut cache = HashMap::new();
    cache.insert(dish.clone(), 0);

    let total_cycles = 1_000_000_000;
    for i in 0..total_cycles {
        rotate_dish(&mut dish);
        if let Some(prev_i) = cache.get(&dish) {
            let cycle = i + 1 - prev_i;
            let remaining = (total_cycles - i - 1) % cycle;
            for _ in 0..remaining {
                rotate_dish(&mut dish);
            }
            break;
        }
        cache.insert(dish.clone(), i + 1);
    }
    Some(calculate_load_northside(&dish))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
