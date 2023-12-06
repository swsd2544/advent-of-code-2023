advent_of_code::solution!(6);

struct RaceInfo {
    time: u64,
    best_distance: u64,
}

impl RaceInfo {
    fn total_winning_methods(&self) -> Option<u64> {
        let a = 1_f64;
        let b = -(self.time as f64);
        let c = self.best_distance as f64;
        let discriminant = (b.powi(2) - (4_f64 * a * c)).sqrt();
        if !discriminant.is_nan() {
            let min_x = ((-b - discriminant) / (2_f64 * a)).floor() as u64;
            let max_x = ((-b + discriminant) / (2_f64 * a)).ceil() as u64;
            Some(max_x - min_x - 1)
        } else {
            None
        }
    }
}

fn parse_input_part_one(input: &str) -> Vec<RaceInfo> {
    let mut times = Vec::new();
    let mut distances = Vec::new();
    input.lines().for_each(|line| {
        if let Some(line) = line.strip_prefix("Time:").map(|line| line.trim()) {
            times = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
        } else if let Some(line) = line.strip_prefix("Distance:").map(|line| line.trim()) {
            distances = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
        }
    });

    let length = times.len().min(distances.len());
    let mut race_infos = Vec::with_capacity(length);
    for idx in 0..length {
        race_infos.push(RaceInfo {
            time: times[idx],
            best_distance: distances[idx],
        });
    }
    race_infos
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input_part_one(input)
            .iter()
            .filter_map(|info| info.total_winning_methods())
            .product(),
    )
}

fn parse_input_part_two(input: &str) -> RaceInfo {
    let mut time = 0;
    let mut best_distance = 0;
    input.lines().for_each(|line| {
        if let Some(line) = line.strip_prefix("Time:") {
            time = line.replace(' ', "").parse().unwrap_or_default();
        } else if let Some(line) = line.strip_prefix("Distance:") {
            best_distance = line.replace(' ', "").parse().unwrap_or_default();
        }
    });

    RaceInfo {
        time,
        best_distance,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_input_part_two(input).total_winning_methods()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
