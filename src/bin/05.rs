advent_of_code::solution!(5);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.start >= other.end || self.end <= other.start {
            None
        } else {
            Some(Self {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        }
    }

    fn difference(&self, other: &Self) -> Option<Self> {
        if other.contains(self) {
            None
        } else if self.start >= other.end || self.end <= other.start {
            Some(self.clone())
        } else if self.start >= other.start {
            Some(Range {
                start: other.end,
                end: self.end,
            })
        } else {
            Some(Range {
                start: self.start,
                end: other.end,
            })
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn shift(&self, offset: i64) -> Self {
        Self {
            start: self.start + offset,
            end: self.end + offset,
        }
    }
}

fn parse_seeds(input: &str) -> Vec<Range> {
    input
        .split_once(' ')
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| {
            let seed = seed.parse().unwrap();
            Range {
                start: seed,
                end: seed + 1,
            }
        })
        .collect()
}

fn parse_seed_ranges(input: &str) -> Vec<Range> {
    let mut ranges = Vec::new();
    let mut tokens = input.split_once(' ').unwrap().1.split_whitespace();
    while let Some(start) = tokens.next() {
        let start = start.parse().unwrap();
        let len = tokens.next().unwrap().parse::<i64>().unwrap();
        ranges.push(Range {
            start,
            end: start + len,
        });
    }
    ranges
}

#[derive(Debug)]
struct RangeMap {
    src: Range,
    dst: Range,
}

fn parse_section(input: &str) -> RangeMap {
    let mut tokens = input.split_whitespace();
    let dst_start = tokens.next().unwrap().parse().unwrap();
    let src_start = tokens.next().unwrap().parse().unwrap();
    let len = tokens.next().unwrap().parse::<i64>().unwrap();
    RangeMap {
        src: Range {
            start: src_start,
            end: src_start + len,
        },
        dst: Range {
            start: dst_start,
            end: dst_start + len,
        },
    }
}

enum Update {
    NoChange,
    Moved(Range),
    TwoSplits {
        unmoved: Range,
        moved: Range,
    },
    ThreeSplits {
        left_unmoved: Range,
        right_unmoved: Range,
        moved: Range,
    },
}

fn apply_map(range: &Range, RangeMap { src, dst }: &RangeMap) -> Update {
    if src.contains(range) {
        Update::Moved(range.shift(dst.start - src.start))
    } else if range.contains(src) {
        let left_unmoved = Range {
            start: range.start,
            end: src.start,
        };
        let right_unmoved = Range {
            start: src.end,
            end: range.end,
        };
        let moved = dst.clone();
        Update::ThreeSplits {
            left_unmoved,
            right_unmoved,
            moved,
        }
    } else if let Some(intersection) = range.intersection(src) {
        let unmoved = range.difference(&intersection).unwrap();
        let moved = intersection.shift(dst.start - src.start);
        Update::TwoSplits { unmoved, moved }
    } else {
        Update::NoChange
    }
}

fn min_location<'a>(mut ranges: Vec<Range>, sections: impl Iterator<Item = &'a str>) -> i64 {
    for section in sections {
        let mut moved = vec![];
        for map in section.lines().skip(1).map(parse_section) {
            let (new_unmoved, new_moved): (Vec<_>, Vec<_>) =
                ranges
                    .into_iter()
                    .fold((Vec::new(), Vec::new()), |mut acc, range| {
                        match apply_map(&range, &map) {
                            Update::NoChange => acc.0.push(range),
                            Update::Moved(range) => acc.1.push(range),
                            Update::TwoSplits { unmoved, moved } => {
                                acc.0.push(unmoved);
                                acc.1.push(moved);
                            }
                            Update::ThreeSplits {
                                left_unmoved,
                                right_unmoved,
                                moved,
                            } => {
                                acc.0.push(left_unmoved);
                                acc.0.push(right_unmoved);
                                acc.1.push(moved);
                            }
                        }
                        acc
                    });
            ranges = new_unmoved;
            moved.extend_from_slice(&new_moved);
        }
        ranges.extend_from_slice(&moved);
    }
    ranges.into_iter().min().unwrap().start
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut input = input.split("\n\n");
    let seeds = input.next().map(parse_seeds).unwrap();
    Some(min_location(seeds, input))
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut input = input.split("\n\n");
    let seeds = input.next().map(parse_seed_ranges).unwrap();
    Some(min_location(seeds, input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
