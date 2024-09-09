//! In order to improve performance, and since cards are in range `2..=14`, I will
//! encode the hand type and the values of each card in an integer.
//!
//! The order of the various hands is the following: 5 > 4 > 3+2 > 3 > 2+2 > 2 > 1
//! I can use `3 bits` to encode the hand type, and `5*4 == 20` bits to describe each value of the hand,
//! allowing for a very fast and simple comparison between different hands, and thus sorting.

use std::{fs, str::FromStr};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Joker;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Normal;

#[derive(Debug, Error)]
#[error("Cannot parse hand {0:?}")]
struct HandParseError(String);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<T>(u32, std::marker::PhantomData<T>);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    One = 0,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<u8> for HandType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::One,
            1 => Self::Pair,
            2 => Self::TwoPair,
            3 => Self::ThreeOfAKind,
            4 => Self::FullHouse,
            5 => Self::FourOfAKind,
            6 => Self::FiveOfAKind,
            _ => panic!("bruh"),
        }
    }
}

impl<T> Hand<T> {
    pub fn hand_type(self) -> HandType {
        ((self.0 >> (5 * 4)) as u8).into()
    }
}

impl FromStr for Hand<Normal> {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value_of_card = |card: char| {
            Some(match card {
                '2'..='9' => card.to_digit(10).unwrap(), // cannot fail
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => return None,
            })
        };

        let err = || HandParseError(s.to_string());

        if s.len() != 5 {
            return Err(err());
        }

        // first obtain all values of the hand. Later use them to derive
        // the hand type
        let mut values = s
            .chars()
            .try_fold(0, |values, card| {
                let v = value_of_card(card);
                v.map(|card_value| values << 4 | card_value)
            })
            .ok_or_else(|| HandParseError(s.to_string()))?;

        // naive, but not too naive
        // process one card at a time. If not seen yet,
        // count the occurrences and modify the hand type accordingly
        let mut processed = ['X'; 5];
        for (i, card) in s.char_indices() {
            if processed.contains(&card) {
                continue;
            }

            let count = 1 + s
                .char_indices()
                .filter(move |&(j, other)| j != i && other == card)
                .count();

            let cur_hand_type = (values >> (5 * 4)) as u8;

            let new_hand_type = match (count, cur_hand_type.into()) {
                (2, HandType::ThreeOfAKind) | (2, HandType::Pair) => cur_hand_type + 1, // adds a pair to current value
                (3, HandType::Pair) => HandType::FullHouse as u8,
                (5, _) => HandType::FiveOfAKind as u8,
                (4, _) => HandType::FourOfAKind as u8,
                (3, _) => HandType::ThreeOfAKind as u8,
                (2, _) => HandType::Pair as u8,
                _ => cur_hand_type,
            };

            values = (values & 0xFFFFF) | (new_hand_type as u32) << (5 * 4);

            processed[i] = card;
        }
        Ok(Hand(values, Default::default()))
    }
}

impl FromStr for Hand<Joker> {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value_of_card = |card: char| {
            Some(match card {
                '2'..='9' => card.to_digit(10).unwrap(), // cannot fail
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => return None,
            })
        };

        let err = || HandParseError(s.to_string());

        if s.len() != 5 {
            return Err(err());
        }

        // first obtain all values of the hand. Later use them to derive
        // the hand type
        let mut values = s
            .chars()
            .try_fold(0, |values, card| {
                let v = value_of_card(card);
                v.map(|card_value| values << 4 | card_value)
            })
            .ok_or_else(|| HandParseError(s.to_string()))?;

        // naive, but not too naive
        // process one card at a time. If not seen yet,
        // count the occurrences and modify the hand type accordingly
        // Now the joker cards become an issue! We do a second pass LATER to only those, and adjust
        // the hand type accordingly
        let mut processed = ['X'; 5];
        for (i, card) in s.char_indices() {
            // Skip already processed card values AND the joker cards. Joker cards need to be
            // considered later, in the second pass
            if card == 'J' || processed.contains(&card) {
                continue;
            }

            // Count same cards (can be joker!)
            let count = 1 + s
                .char_indices()
                .filter(move |&(j, other)| j != i && other == card)
                .count();

            let cur_hand_type = (values >> (5 * 4)) as u8;

            let new_hand_type = match (count, cur_hand_type.into()) {
                (2, HandType::ThreeOfAKind) | (2, HandType::Pair) => cur_hand_type + 1, // adds a pair to current value
                (3, HandType::Pair) => HandType::FullHouse as u8,
                (5, _) => HandType::FiveOfAKind as u8,
                (4, _) => HandType::FourOfAKind as u8,
                (3, _) => HandType::ThreeOfAKind as u8,
                (2, _) => HandType::Pair as u8,
                _ => cur_hand_type,
            };

            values = (values & 0xFFFFF) | (new_hand_type as u32) << (5 * 4);

            processed[i] = card;
        }

        let joker_count = s.chars().filter(move |&c| c == 'J').count();
        let cur_hand_type = ((values >> (5 * 4)) as u8).into();

        let joker_hand_type = match (joker_count, cur_hand_type) {
            (0, a) => a,
            // One joker: cur_hand_type cannot be FullHouse or FiveOfAKind
            (1, HandType::FourOfAKind) => HandType::FiveOfAKind,
            (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
            (1, HandType::TwoPair) => HandType::FullHouse,
            (1, HandType::Pair) => HandType::ThreeOfAKind,
            (1, HandType::One) => HandType::Pair,
            // Two jokers: cur_hand_type cannot be 2+3, 5, 4, 2+2
            (2, HandType::ThreeOfAKind) => HandType::FiveOfAKind,
            (2, HandType::Pair) => HandType::FourOfAKind,
            (2, HandType::One) => HandType::ThreeOfAKind,
            // Three jokers: cur_hand_type cannot be 2+3, 5, 4, 3, 2+2
            (3, HandType::Pair) => HandType::FiveOfAKind,
            (3, HandType::One) => HandType::FourOfAKind,
            // 4 jokers allow only one other card, easy
            (4, HandType::One) => HandType::FiveOfAKind,
            // hand type does not allow encoding zero cards... Oh well :D
            (5, _) => HandType::FiveOfAKind,
            _ => {
                unreachable!("All other combinations are impossible, since there are only 5 cards")
            }
        };

        values = (values & 0xFFFFF) | (joker_hand_type as u32) << (5 * 4);

        Ok(Hand(values, Default::default()))
    }
}

fn main() {
    let content = fs::read_to_string("inputs/p7/hands.txt").expect("reading input file");

    let mut hands_bids: Vec<(Hand<Normal>, u32)> = content
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| line.next().zip(line.next()).unwrap())
        .map(|(hand_str, bid_str)| (Hand::from_str(hand_str).unwrap(), bid_str.parse().unwrap()))
        .collect();

    hands_bids.sort_by_key(|(hand, _)| *hand);

    // multiply bid by rank (weakest is rank 1, increasing)
    let tot_score: u32 = hands_bids
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as u32 + 1) * *bid)
        .sum();
    println!("[PART 1] Sum of (rank * bid) = {tot_score}");

    let mut hands_bids: Vec<(Hand<Joker>, u32)> = content
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| line.next().zip(line.next()).unwrap())
        .map(|(hand_str, bid_str)| (Hand::from_str(hand_str).unwrap(), bid_str.parse().unwrap()))
        .collect();

    hands_bids.sort_by_key(|(hand, _)| *hand);

    // multiply bid by rank (weakest is rank 1, increasing)
    let tot_score: u32 = hands_bids
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as u32 + 1) * *bid)
        .sum();
    println!("[PART 2] This is horrible! Result = {tot_score}");
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Hand, HandType};

    #[test]
    fn hand_from_str() {
        let values = &[
            ("", None),
            ("2", None),
            ("11111", None),
            ("22221", None),
            (
                "22222",
                Some(Hand(((HandType::FiveOfAKind as u32) << 5 * 4) | 0x22222)),
            ),
            (
                "99999",
                Some(Hand(((HandType::FiveOfAKind as u32) << 5 * 4) | 0x99999)),
            ),
            (
                "AAAAA",
                Some(Hand(((HandType::FiveOfAKind as u32) << 5 * 4) | 0xEEEEE)),
            ),
            (
                "AAAA2",
                Some(Hand(((HandType::FourOfAKind as u32) << 5 * 4) | 0xEEEE2)),
            ),
            (
                "22233",
                Some(Hand(((HandType::FullHouse as u32) << 5 * 4) | 0x22233)),
            ),
            (
                "22333",
                Some(Hand(((HandType::FullHouse as u32) << 5 * 4) | 0x22333)),
            ),
        ][..];

        for &(input, expected) in values {
            let actual = Hand::from_str(input).ok();
            assert_eq!(
                expected, actual,
                "parsing hand {:?} did not produce {:x?} as expected, but instead I got {:x?}",
                input, expected, actual
            );
        }
    }
}
