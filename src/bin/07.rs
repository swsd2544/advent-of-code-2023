use std::cmp::Ordering;

advent_of_code::solution!(7);

fn map_card_to_value(c: char) -> Option<usize> {
    match c {
        '2' => Some(0),
        '3' => Some(1),
        '4' => Some(2),
        '5' => Some(3),
        '6' => Some(4),
        '7' => Some(5),
        '8' => Some(6),
        '9' => Some(7),
        'T' => Some(8),
        'J' => Some(9),
        'Q' => Some(10),
        'K' => Some(11),
        'A' => Some(12),


        _ => None,
    }
}

fn map_value_to_card(v: usize) -> Option<char> {
    match v {
        0 => Some('2'),
        1 => Some('3'),
        2 => Some('4'),
        3 => Some('5'),
        4 => Some('6'),
        5 => Some('7'),
        6 => Some('8'),
        7 => Some('9'),
        8 => Some('T'),
        9 => Some('J'),
        10 => Some('Q'),
        11 => Some('K'),
        12 => Some('A'),
        _ => None,
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[char]> for HandType {
    fn from(value: &[char]) -> Self {
        let mut count = [0; 13];
        for c in value.iter().filter_map(|&c| map_card_to_value(c)) {
            count[c] += 1;
        }

        let mut three_of_a_kind = false;
        let mut pairs = 0;

        for &n in count.iter() {
            match n {
                5 => return Self::FiveOfAKind,
                4 => return Self::FourOfAKind,
                3 => three_of_a_kind = true,
                2 => pairs += 1,

                _ => {}
            }
        }

        if three_of_a_kind && pairs == 1 {
            Self::FullHouse
        } else if three_of_a_kind {
            Self::ThreeOfAKind
        } else if pairs == 2 {
            Self::TwoPairs
        } else if pairs == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Eq, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    hand_type: HandType,
    joker: bool,
}

impl Hand {
    fn convert_joker(&mut self) {
        let mut count = [0; 13];
        for &c in self.cards.iter().filter(|&&c| c != 'J') {
            if let Some(v) = map_card_to_value(c) {
                count[v] += 1;
            }
        }

        if let Some(max_idx) = count
            .iter()
            .enumerate()
            .max_by_key(|&(_, &count)| count)
            .map(|(idx, _)| idx)
        {
            let max_char = map_value_to_card(max_idx).unwrap();
            self.cards.iter_mut().for_each(|char| {
                if *char == 'J' {
                    *char = max_char;
                }
            });

            self.hand_type = HandType::from(self.cards.as_slice());
            self.joker = true;
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (hand, bid_str) = value
            .split_once(' ')
            .ok_or("Invalid input string".to_string())?;
        let cards: Vec<_> = hand.chars().collect();
        let bid = bid_str.parse::<u32>().map_err(|err| err.to_string())?;
        let hand_type = HandType::from(cards.as_slice());
        Ok(Self {
            cards,
            bid,
            hand_type,
            joker: false,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards.len() != other.cards.len() {
            return Ordering::Equal;
        }

        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        let joker_v = map_card_to_value('J').unwrap() as isize;

        for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
            let self_v = map_card_to_value(*self_card).map(|v| {
                if v as isize == joker_v {
                    -1
                } else {
                    v as isize
                }
            });
            let other_v = map_card_to_value(*other_card).map(|v| {
                if v as isize == joker_v {
                    -1
                } else {
                    v as isize
                }
            });

            let cmp_res = self_v.cmp(&other_v);
            if cmp_res != Ordering::Equal {
                return cmp_res;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.cards.len() != other.cards.len() {
            return false;
        }
        for idx in 0..self.cards.len() {
            if self.cards[idx] != other.cards[idx] {
                return false;
            }
        }
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| Hand::try_from(line).unwrap())
        .collect::<Vec<_>>();
    hands.sort();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| Hand::try_from(line).unwrap())
        .collect::<Vec<_>>();
    hands.iter_mut().for_each(|hand| hand.convert_joker());
    hands.sort();

    Some(
        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));

    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}

