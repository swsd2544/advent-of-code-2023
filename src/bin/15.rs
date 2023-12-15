advent_of_code::solution!(15);

const TOTAL_BOXES: usize = 256;

#[derive(Clone)]
struct Len {
    label: String,
    focal_length: usize,
}

enum Operation {
    Remove(String),
    Insert(Len),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let mut tokens = value.split(['-', '=']);
        if value.contains('-') {
            let label = tokens.next().unwrap();
            Self::Remove(label.to_string())
        } else if value.contains('=') {
            let label = tokens.next().unwrap();
            let focal_length = tokens.next().unwrap().parse().unwrap();
            Self::Insert(Len {
                label: label.to_string(),
                focal_length,
            })
        } else {
            panic!("Unknown operation")
        }
    }
}

impl Operation {
    fn calculate_box_idx(&self) -> usize {
        let input = match self {
            Self::Remove(label) => label.as_str(),
            Self::Insert(Len {
                label,
                focal_length: _,
            }) => label.as_str(),
        };
        calculate_str_hash(input) as usize
    }
}

fn calculate_str_hash(input: &str) -> u32 {
    let mut current_value = 0;
    for c in input.chars() {
        current_value = (current_value + c as u32) * 17 % TOTAL_BOXES as u32;
    }
    current_value
}

pub fn part_one(input: &str) -> Option<u32> {
    let hash_sum = input.trim().split(',').map(calculate_str_hash).sum::<u32>();
    Some(hash_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<Vec<Len>> = vec![vec![]; TOTAL_BOXES];
    input
        .trim()
        .split(',')
        .map(Operation::from)
        .for_each(|operation| {
            let box_idx = operation.calculate_box_idx();
            match operation {
                Operation::Remove(label) => {
                    if let Some(len_idx) = boxes[box_idx].iter().position(|l| l.label.eq(&label)) {
                        boxes[box_idx].remove(len_idx);
                    }
                }
                Operation::Insert(len) => {
                    if let Some(len_idx) =
                        boxes[box_idx].iter().position(|l| l.label.eq(&len.label))
                    {
                        boxes[box_idx][len_idx] = len;
                    } else {
                        boxes[box_idx].push(len);
                    }
                }
            }
        });
    let focusing_power = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, lens)| {
            lens.iter()
                .enumerate()
                .map(|(len_idx, len)| (box_idx + 1) * (len_idx + 1) * len.focal_length)
                .sum::<usize>()
        })
        .sum::<usize>();
    Some(focusing_power as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
