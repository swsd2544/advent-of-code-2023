use std::collections::HashMap;

advent_of_code::solution!(16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Beam {
    col_idx: usize,
    row_idx: usize,
    direction: Direction,
}

enum BeamUpdate {
    NoChange,
    Reflected(Direction),
    Splited(Direction, Direction),
}

impl Beam {
    fn interact(self, object: char) -> BeamUpdate {
        use Direction::*;
        match object {
            '.' => BeamUpdate::NoChange,
            '/' => BeamUpdate::Reflected(match self.direction {
                Up => Right,
                Down => Left,
                Left => Down,
                Right => Up,
            }),
            '\\' => BeamUpdate::Reflected(match self.direction {
                Up => Left,
                Down => Right,
                Left => Up,
                Right => Down,
            }),
            '|' if matches!(self.direction, Right | Left) => BeamUpdate::Splited(Up, Down),
            '|' if matches!(self.direction, Up | Down) => BeamUpdate::NoChange,
            '-' if matches!(self.direction, Up | Down) => BeamUpdate::Splited(Left, Right),
            '-' if matches!(self.direction, Right | Left) => BeamUpdate::NoChange,
            c => panic!("unknown object: {}", c),
        }
    }

    fn bounces(&mut self, max_row_idx: usize, max_col_idx: usize) -> bool {
        use Direction::*;
        match self.direction {
            Up => {
                self.row_idx > 0 && {
                    self.row_idx -= 1;
                    true
                }
            }
            Down => {
                self.row_idx < max_row_idx - 1 && {
                    self.row_idx += 1;
                    true
                }
            }
            Left => {
                self.col_idx > 0 && {
                    self.col_idx -= 1;
                    true
                }
            }
            Right => {
                self.col_idx < max_col_idx - 1 && {
                    self.col_idx += 1;
                    true
                }
            }
        }
    }
}

fn count_energized_tiles(grid: &Vec<Vec<char>>, starter: Beam) -> u32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut energized_tiles = HashMap::new();

    let mut beams = vec![starter];
    while let Some(mut beam) = beams.pop() {
        loop {
            let entry = energized_tiles
                .entry((beam.row_idx, beam.col_idx))
                .or_insert_with(Vec::new);

            if entry.contains(&beam.direction) {
                break;
            }

            entry.push(beam.direction);

            match beam.interact(grid[beam.row_idx][beam.col_idx]) {
                BeamUpdate::NoChange => {}
                BeamUpdate::Reflected(direction) => beam.direction = direction,
                BeamUpdate::Splited(direction, another_direction) => {
                    let mut another_beam = Beam {
                        row_idx: beam.row_idx,
                        col_idx: beam.col_idx,
                        direction: another_direction,
                    };

                    if another_beam.bounces(num_rows, num_cols) {
                        beams.push(another_beam);
                    }

                    beam.direction = direction;
                }
            }

            if !beam.bounces(num_rows, num_cols) {
                break;
            }
        }
    }

    energized_tiles.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let energized_tiles = count_energized_tiles(
        &grid,
        Beam {
            row_idx: 0,
            col_idx: 0,
            direction: Direction::Right,
        },
    );
    Some(energized_tiles)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut positions = vec![];
    for row_idx in 0..grid.len() {
        for col_idx in 0..grid[0].len() {
            if row_idx == 0 {
                positions.push(Beam {
                    row_idx,
                    col_idx,
                    direction: Direction::Down,
                });
            } else if row_idx == grid.len() - 1 {
                positions.push(Beam {
                    row_idx,
                    col_idx,
                    direction: Direction::Up,
                });
            }

            if col_idx == 0 {
                positions.push(Beam {
                    row_idx,
                    col_idx,
                    direction: Direction::Right,
                });
            } else if col_idx == grid[0].len() - 1 {
                positions.push(Beam {
                    row_idx,
                    col_idx,
                    direction: Direction::Left,
                });
            }
        }
    }

    positions
        .into_iter()
        .map(|pos| count_energized_tiles(&grid, pos))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
