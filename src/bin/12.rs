advent_of_code::solution!(12);

#[derive(Clone, Copy, PartialEq)]
enum Spring {
    Ok,
    Broken,
    Unknown,
}

impl From<u8> for Spring {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Ok,
            b'#' => Self::Broken,
            b'?' => Self::Unknown,
            _ => panic!("invalid token"),
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    None,
    Invalid,
    Valid(u64),
}

impl Outcome {
    fn value(&self) -> Option<u64> {
        if let Self::Valid(outcome) = self {
            Some(*outcome)
        } else {
            None
        }
    }
}

fn place(len: usize, springs: &[Spring], lens: &[usize], m: &mut Vec<Vec<Outcome>>) -> Outcome {
    if len > springs.len() || springs[..len].iter().any(|spring| *spring == Spring::Ok) {
        Outcome::Invalid
    } else if len == springs.len() {
        arrangements(&springs[len..], lens, m)
    } else if springs[len] == Spring::Broken {
        Outcome::Invalid
    } else {
        arrangements(&springs[len + 1..], lens, m)
    }
}

fn arrangements(springs: &[Spring], lens: &[usize], m: &mut Vec<Vec<Outcome>>) -> Outcome {
    if let memo @ (Outcome::Valid(_) | Outcome::Invalid) = m[springs.len()][lens.len()] {
        return memo;
    }
    let outcome = match (springs.iter().next(), lens.iter().next()) {
        (Some(Spring::Ok), _) => arrangements(&springs[1..], lens, m),
        (Some(Spring::Broken), None) => Outcome::Invalid,
        (Some(Spring::Broken), Some(&len)) => place(len, springs, &lens[1..], m),
        (Some(Spring::Unknown), None) => arrangements(&springs[1..], lens, m),
        (Some(Spring::Unknown), Some(&len)) => {
            let here = place(len, springs, &lens[1..], m).value();
            let there = arrangements(&springs[1..], lens, m).value();
            match (here, there) {
                (Some(n), Some(m)) => Outcome::Valid(n + m),
                (Some(n), None) => Outcome::Valid(n),
                (None, Some(m)) => Outcome::Valid(m),
                (None, None) => Outcome::Invalid,
            }
        }
        (None, Some(_)) => Outcome::Invalid,
        (None, None) => Outcome::Valid(1),
    };
    m[springs.len()][lens.len()] = outcome;
    outcome
}

fn arrangements_memoized((springs, lens): (Vec<Spring>, Vec<usize>)) -> u64 {
    let mut m = vec![vec![Outcome::None; lens.len() + 1]; springs.len() + 1];
    arrangements(&springs, &lens, &mut m)
        .value()
        .unwrap_or_default()
}

fn expand((mut springs, lens): (Vec<Spring>, Vec<usize>)) -> (Vec<Spring>, Vec<usize>) {
    springs.push(Spring::Unknown);
    let springs_length = springs.len() * 5 - 1;
    (
        springs.into_iter().cycle().take(springs_length).collect(),
        lens.repeat(5),
    )
}

fn parse_input(input: &str) -> (Vec<Spring>, Vec<usize>) {
    let (springs, lens) = input.split_once(' ').unwrap();
    let springs = springs.bytes().map(Spring::from).collect();
    let lens = lens.split(',').map(|len| len.parse().unwrap()).collect();
    (springs, lens)
}

pub fn part_one(input: &str) -> Option<u64> {
    let total_arrangments = input
        .lines()
        .map(parse_input)
        .map(arrangements_memoized)
        .sum();
    Some(total_arrangments)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total_arrangments = input
        .lines()
        .map(parse_input)
        .map(expand)
        .map(arrangements_memoized)
        .sum();
    Some(total_arrangments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
