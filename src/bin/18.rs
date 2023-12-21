advent_of_code::solution!(18);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<usize> {
    let directions = input.lines().map(|line| {
        let mut tokens = line.split_whitespace();
        let direction = tokens.next().unwrap();
        let length = tokens.next().unwrap().parse::<isize>().unwrap();
        match direction {
            "R" => (Direction::Right, length),
            "L" => (Direction::Left, length),
            "U" => (Direction::Up, length),
            "D" => (Direction::Down, length),
            c => panic!("unknown direction: {}", c),
        }
    });

    let mut curr_point = (0, 0);
    let mut prev_point = (0, 0);
    let mut outer_points = 0;
    let mut area = 0;

    for (direction, length) in directions {
        match direction {
            Direction::Up => curr_point.0 += length,
            Direction::Down => curr_point.0 -= length,
            Direction::Left => curr_point.1 -= length,
            Direction::Right => curr_point.1 += length,
        };

        area += curr_point.0 * prev_point.1;
        area -= curr_point.1 * prev_point.0;
        outer_points += length as usize;
        prev_point = curr_point;
    }

    let area = area.unsigned_abs() / 2 + outer_points / 2 + 1;
    Some(area)
}

pub fn part_two(input: &str) -> Option<usize> {
    let directions = input.lines().map(|line| {
        let tokens = line.split_whitespace();
        let mut length_direction_encoded = tokens
            .last()
            .unwrap()
            .chars()
            .filter(|c| c.is_alphanumeric());
        let mut hexadecimal_length = String::new();
        for _ in 0..5 {
            hexadecimal_length.push(length_direction_encoded.next().unwrap());
        }
        let length = isize::from_str_radix(&hexadecimal_length, 16).unwrap();
        let direction = match length_direction_encoded.next().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            c => panic!("unknown encoded direction: {}", c),
        };
        (direction, length)
    });

    let mut curr_point = (0, 0);
    let mut prev_point = (0, 0);
    let mut outer_points = 0;
    let mut area = 0;

    for (direction, length) in directions {
        match direction {
            Direction::Up => curr_point.0 += length,
            Direction::Down => curr_point.0 -= length,
            Direction::Left => curr_point.1 -= length,
            Direction::Right => curr_point.1 += length,
        };

        area += curr_point.0 * prev_point.1;
        area -= curr_point.1 * prev_point.0;
        outer_points += length as usize;
        prev_point = curr_point;
    }

    let area = area.unsigned_abs() / 2 + outer_points / 2 + 1;
    Some(area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
