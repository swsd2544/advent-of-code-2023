use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    num::NonZeroUsize,
};

advent_of_code::solution!(17);

type Position = (usize, usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    pos: Position,
    direction: Direction,
    direction_count: usize,
}

fn get_neighbors(
    node: &Node,
    grid: &Vec<Vec<u32>>,
    min_blocks: NonZeroUsize,
    max_blocks: NonZeroUsize,
) -> Vec<Node> {
    let mut neighbors = Vec::new();

    let min_blocks = min_blocks.get();
    let max_blocks = max_blocks.get();

    if node.direction != Direction::Down && node.pos.0 > 0 {
        let pos = (node.pos.0 - 1, node.pos.1);
        let direction = Direction::Up;
        let direction_count = if node.direction == direction {
            node.direction_count + 1
        } else {
            1
        };
        if node.direction == Direction::None
            || ((node.direction == direction || node.direction_count >= min_blocks)
                && direction_count <= max_blocks)
        {
            neighbors.push(Node {
                pos,
                direction,
                direction_count,
            });
        }
    }

    if node.direction != Direction::Up && node.pos.0 + 1 < grid.len() {
        let pos = (node.pos.0 + 1, node.pos.1);
        let direction = Direction::Down;
        let direction_count = if node.direction == direction {
            node.direction_count + 1
        } else {
            1
        };
        if node.direction == Direction::None
            || ((node.direction == direction || node.direction_count >= min_blocks)
                && direction_count <= max_blocks)
        {
            neighbors.push(Node {
                pos,
                direction,
                direction_count,
            });
        }
    }

    if node.direction != Direction::Right && node.pos.1 > 0 {
        let pos = (node.pos.0, node.pos.1 - 1);
        let direction = Direction::Left;
        let direction_count = if node.direction == direction {
            node.direction_count + 1
        } else {
            1
        };
        if node.direction == Direction::None
            || ((node.direction == direction || node.direction_count >= min_blocks)
                && direction_count <= max_blocks)
        {
            neighbors.push(Node {
                pos,
                direction,
                direction_count,
            });
        }
    }

    if node.direction != Direction::Left && node.pos.1 + 1 < grid[0].len() {
        let pos = (node.pos.0, node.pos.1 + 1);
        let direction = Direction::Right;
        let direction_count = if node.direction == direction {
            node.direction_count + 1
        } else {
            1
        };
        if node.direction == Direction::None
            || ((node.direction == direction || node.direction_count >= min_blocks)
                && direction_count <= max_blocks)
        {
            neighbors.push(Node {
                pos,
                direction,
                direction_count,
            });
        }
    }

    neighbors
}

#[derive(PartialEq, Eq)]
struct State {
    node: Node,
    cost: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .then(other.node.pos.cmp(&self.node.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let start_pos = (0, 0);
    let end_pos = (grid.len() - 1, grid[0].len() - 1);

    let mut min_costs: HashMap<Node, u32> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    queue.push(Reverse(State {
        node: Node {
            pos: start_pos,
            direction: Direction::None,
            direction_count: 0,
        },
        cost: 0,
    }));

    while let Some(Reverse(state)) = queue.pop() {
        if state.node.pos == end_pos {
            return Some(state.cost);
        }

        let neighbors = get_neighbors(
            &state.node,
            &grid,
            NonZeroUsize::new(1).unwrap(),
            NonZeroUsize::new(3).unwrap(),
        );
        for node in neighbors {
            let cost = state.cost + grid[node.pos.0][node.pos.1];
            if min_costs.get(&node).is_some_and(|c| *c <= cost) {
                continue;
            }
            min_costs.insert(node, cost);
            queue.push(Reverse(State { node, cost }));
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let start_pos = (0, 0);
    let end_pos = (grid.len() - 1, grid[0].len() - 1);

    let mut min_costs: HashMap<Node, u32> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    queue.push(Reverse(State {
        node: Node {
            pos: start_pos,
            direction: Direction::None,
            direction_count: 0,
        },
        cost: 0,
    }));

    while let Some(Reverse(state)) = queue.pop() {
        if state.node.pos == end_pos && state.node.direction_count >= 4 {
            return Some(state.cost);
        }

        let neighbors = get_neighbors(
            &state.node,
            &grid,
            NonZeroUsize::new(4).unwrap(),
            NonZeroUsize::new(10).unwrap(),
        );
        for node in neighbors {
            let cost = state.cost + grid[node.pos.0][node.pos.1];
            if min_costs.get(&node).is_some_and(|c| *c <= cost) {
                continue;
            }
            min_costs.insert(node, cost);
            queue.push(Reverse(State { node, cost }));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn another_test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
