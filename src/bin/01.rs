advent_of_code::solution!(1);

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn extract_digit(input: &str) -> Option<u32> {
    input
        .chars()
        .next()
        .and_then(|c| if c.is_numeric() { c.to_digit(10) } else { None })
}

fn extract_digit_with_word(input: &str) -> Option<u32> {
    DIGITS
        .iter()
        .enumerate()
        .find_map(|(idx, digit)| input.starts_with(digit).then_some(idx as u32))
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let (mut first_number, mut last_number) = (None, None);
            (0..line.len())
                .map(|idx| &line[idx..])
                .filter_map(extract_digit)
                .for_each(|digit| {
                    first_number = first_number.or(Some(digit));
                    last_number = Some(digit);
                });
            first_number.unwrap_or_default() * 10 + last_number.unwrap_or_default()
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let (mut first_number, mut last_number) = (None, None);
            (0..line.len())
                .map(|idx| &line[idx..])
                .filter_map(|s| extract_digit_with_word(s).or(extract_digit(s)))
                .for_each(|digit| {
                    first_number = first_number.or(Some(digit));
                    last_number = Some(digit);
                });
            first_number.unwrap_or_default() * 10 + last_number.unwrap_or_default()
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
